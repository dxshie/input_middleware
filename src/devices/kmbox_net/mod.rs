use std::{mem::MaybeUninit, net::SocketAddr};

use log::{debug, error, info};
use socket2::{Domain, Protocol, Socket, Type};

use crate::devices::kmbox_net::{
    cmd::CMD,
    errors::{KMBoxNetConnectionError, KMBoxNetSendError},
    structs::CmdData,
};

use self::{
    button_state::{ButtonState, MwheelState},
    keyboard::KeyboardKey,
    structs::ClientTx,
};

pub mod button_state;
pub mod cmd;
mod cmd_instruction;
pub mod errors;
mod key_instructions;
pub mod keyboard;
mod structs;

fn to_hex(src: &str, len: usize) -> u32 {
    let mut dest: [u32; 16] = [0; 16];
    for i in 0..len {
        let bytes = src.as_bytes();
        let h1 = bytes[2 * i] as char;
        let h2 = bytes[2 * i + 1] as char;
        let mut s1 = h1.to_ascii_uppercase() as u32 - 0x30;
        if s1 > 9 {
            s1 -= 7;
        }
        let mut s2 = h2.to_ascii_uppercase() as u32 - 0x30;
        if s2 > 9 {
            s2 -= 7;
        }
        dest[i] = s1 * 16 + s2;
    }
    dest[0] << 24 | dest[1] << 16 | dest[2] << 8 | dest[3]
}

#[derive(Debug)]
pub struct KMBoxNet {
    socket: Socket,
    socket_addr: SocketAddr,
    rx: MaybeUninit<structs::ClientTx>,
    tx: MaybeUninit<structs::ClientTx>,
}

impl KMBoxNet {
    pub fn new(ip: String, port: u16, uuid: String) -> Result<Self, KMBoxNetConnectionError> {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).unwrap();
        let socket_addr = SocketAddr::new(ip.parse().unwrap(), port);
        let rand = rand::random::<u32>();
        let tx = ClientTx {
            head: structs::CmdHead {
                mac: to_hex(uuid.as_str(), 4),
                rand,
                indexpts: 0,
                cmd: CMD::CONNECT.into(),
            },
            data: structs::CmdData { u8buff: [0; 1024] },
        };
        debug!("Connecting to KMBox Net\n{:#?}", tx.head);
        socket
            .set_read_timeout(Some(std::time::Duration::from_secs(3)))
            .unwrap();
        socket
            .set_write_timeout(Some(std::time::Duration::from_secs(3)))
            .unwrap();
        socket
            .send_to(
                unsafe {
                    std::slice::from_raw_parts(
                        &tx as *const structs::ClientTx as *const u8,
                        std::mem::size_of::<structs::ClientTx>(),
                    )
                },
                &socket_addr.into(),
            )
            .map_err(KMBoxNetConnectionError)?; // error handling inform that user should check if is uuid, port, ip is correct
        let mut rx = MaybeUninit::<ClientTx>::uninit();
        socket
            .recv_from(unsafe {
                std::slice::from_raw_parts_mut(
                    rx.as_mut_ptr() as *mut _,
                    std::mem::size_of::<structs::ClientTx>(),
                )
            })
            .map_err(KMBoxNetConnectionError)?;
        let rx = unsafe { rx.assume_init() };

        if rx.head.cmd != tx.head.cmd || rx.head.indexpts != tx.head.indexpts {
            error!("Connect failed");
            return Err(KMBoxNetConnectionError(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                "Connect failed response does not match request",
            )));
        }
        info!("KMBox Net connected");

        Ok(KMBoxNet {
            socket,
            socket_addr,
            tx: MaybeUninit::new(tx),
            rx: MaybeUninit::new(rx),
        })
    }

    pub fn set_timeout(&mut self, timeout: std::time::Duration) -> Result<(), std::io::Error> {
        self.socket.set_read_timeout(Some(timeout))?;
        self.socket.set_write_timeout(Some(timeout))?;
        debug!("Timeout set to {:?}", timeout);
        Ok(())
    }

    pub fn keyboard_keydown(&mut self, key: KeyboardKey) -> Result<(), KMBoxNetSendError> {
        let tx = unsafe { self.tx.assume_init_mut() };
        let key = key as u8;
        // for i in 0..10 {
        unsafe {
            tx.data.cmd_keyboard.button[0] = key as char;
        }
        // }
        info!("Keyboard key set tx\n{:?}", unsafe { tx.data.cmd_keyboard });
        self.send(CMD::KEYBOARD_ALL)?;
        Ok(())
    }

    pub fn keyboard_keyup(&mut self, key: KeyboardKey) -> Result<(), KMBoxNetSendError> {
        let tx = unsafe { self.tx.assume_init_mut() };
        let key = key as u8;
        // for i in 0..10 {
        unsafe {
            if tx.data.cmd_keyboard.button[0] == key as char {
                tx.data.cmd_keyboard.button[0] = 0 as char;
            }
        }
        // }
        info!("Keyboard key set tx\n{:?}", unsafe { tx.data.cmd_keyboard });
        self.send(CMD::KEYBOARD_ALL)?;
        Ok(())
    }

    pub fn mouse_left_click(
        &mut self,
        state: impl Into<ButtonState>,
    ) -> Result<(), KMBoxNetSendError> {
        let tx = unsafe { self.tx.assume_init_mut() };
        tx.data.cmd_mouse.button = Into::into(state.into());
        info!("Mouse left button set tx\n{:?}", unsafe {
            tx.data.cmd_mouse
        });
        self.send(CMD::MOUSE_LEFT)?;
        Ok(())
    }

    pub fn mouse_right_click(
        &mut self,
        state: impl Into<ButtonState>,
    ) -> Result<(), KMBoxNetSendError> {
        let tx = unsafe { self.tx.assume_init_mut() };
        tx.data.cmd_mouse.button = Into::into(state.into());
        info!("Mouse right button set tx\n{:?}", unsafe {
            tx.data.cmd_mouse
        });
        self.send(CMD::MOUSE_RIGHT)?;
        Ok(())
    }

    pub fn mouse_wheel(&mut self, state: impl Into<MwheelState>) -> Result<(), KMBoxNetSendError> {
        let tx = unsafe { self.tx.assume_init_mut() };
        tx.data.cmd_mouse.wheel = Into::into(state.into());
        info!("Mouse wheel set tx\n{:?}", unsafe { tx.data.cmd_mouse });
        self.send(CMD::MOUSE_WHEEL)?;
        Ok(())
    }

    /// Move the mouse to the specified position relative to the current position
    /// +x is right, +y is down
    pub fn mouse_move(&mut self, position: impl Into<[i32; 2]>) -> Result<(), KMBoxNetSendError> {
        let tx = unsafe { self.tx.assume_init_mut() };
        let data = position.into();
        tx.data.cmd_mouse.x = data[0];
        tx.data.cmd_mouse.y = data[1];
        info!("Mouse move set tx\n{:?}", unsafe { tx.data.cmd_mouse });
        self.send(CMD::MOUSE_MOVE)?;
        Ok(())
    }

    /// Reboot the KMBoxNet
    pub fn reboot(&mut self) -> Result<(), KMBoxNetSendError> {
        debug!("Rebooting KMBoxNet");
        self.send(CMD::REBOOT)?;
        Ok(())
    }

    /// # Safety
    /// This function is unsafe because it dereferences a raw pointer.
    /// The caller must ensure that the pointer is valid.
    /// The Pointer is valid when the struct is constructed the tx is init
    fn send(&mut self, cmd: CMD) -> Result<(), KMBoxNetSendError> {
        let tx = unsafe { self.tx.assume_init_mut() };
        tx.head.indexpts += 1;
        tx.head.cmd = cmd.into();
        tx.head.rand = rand::random::<u32>();
        debug!("Send command tx.head\n{:?}", tx.head);
        unsafe {
            let CmdData { cmd_mouse } = tx.data;
            {
                debug!("Send data tx.data\n{:?}", cmd_mouse);
            }
        }
        self.socket
            .send_to(
                unsafe {
                    std::slice::from_raw_parts(
                        tx as *const structs::ClientTx as *const u8,
                        std::mem::size_of::<structs::ClientTx>(),
                    )
                },
                &self.socket_addr.into(),
            )
            .map_err(KMBoxNetSendError)?;
        self.socket
            .recv_from(unsafe {
                std::slice::from_raw_parts_mut(
                    self.rx.as_mut_ptr() as *mut _,
                    std::mem::size_of::<structs::ClientTx>(),
                )
            })
            .map_err(KMBoxNetSendError)?;
        let rx = unsafe { self.rx.assume_init() };
        info!(
            "Command send successful rx res\n{:?}\n{:?}",
            rx.head,
            unsafe { rx.data.cmd_mouse }
        );
        Ok(())
    }
}
