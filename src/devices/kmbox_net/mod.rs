use std::{
    mem::MaybeUninit,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use log::{debug, error, info};
use socket2::{Domain, Protocol, Socket, Type};

use crate::{
    button_state::{ButtonState, MwheelState},
    devices::kmbox_net::{
        cmd::CMD,
        errors::{KMBoxNetConnectionError, KMBoxNetSendError},
        structs::CmdData,
    },
    keyboardkeys::KeyboardKey,
    InputMiddlewareDeviceAction,
};

use self::structs::{ClientTx, MonitorData};

pub mod cmd;
mod cmd_instruction;
pub mod errors;
mod key_instructions;
mod keyboard;
pub(crate) mod structs;

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

/// T can be ClientTx or MonitorData
#[derive(Debug)]
pub struct KMBoxNet {
    socket: Socket,
    socket_addr: SocketAddr,
    rx: MaybeUninit<ClientTx>,
    tx: MaybeUninit<ClientTx>,
}

#[derive(Debug)]
pub struct KMBoxNetMonitor {
    socket: Socket,
    socket_addr: SocketAddr,
    monitor: MaybeUninit<MonitorData>,
}

#[derive(Debug, Clone)]
pub struct KMBoxNetConfig {
    pub ip: String,
    pub port: u16,
    pub uuid: String,
}

impl Default for KMBoxNetConfig {
    fn default() -> Self {
        Self {
            ip: "192.168.2.188".into(),
            port: 16824,
            uuid: "XXXXXXXX".into(),
        }
    }
}

impl KMBoxNetConfig {
    pub fn default_with_uuid(uuid: &str) -> Self {
        Self::default().set_uuid(uuid.into())
    }

    pub fn new(ip: &str, port: u16, uuid: &str) -> Self {
        Self {
            ip: ip.into(),
            port,
            uuid: uuid.into(),
        }
    }

    pub fn set_uuid(mut self, uuid: String) -> Self {
        self.uuid = uuid;
        self
    }
}

impl KMBoxNetMonitor {
    pub fn new(socket_addr: SocketAddr) -> Self {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).unwrap();
        let mut socket_addr = SocketAddr::from(socket_addr);
        socket_addr.set_port(socket_addr.port() + 1);
        socket_addr.set_ip(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
        Self {
            socket: socket,
            socket_addr: socket_addr,
            monitor: MaybeUninit::new(MonitorData::default()),
        }
    }

    pub fn bind(&mut self) -> Result<(), KMBoxNetConnectionError> {
        debug!("Bind local Monitor for KMBoxNet at {:?}", self.socket_addr);
        self.socket
            .set_read_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        self.socket
            .set_write_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        self.socket
            .bind(&self.socket_addr.into())
            .map_err(KMBoxNetConnectionError)?;
        Ok(())
    }

    pub fn recv_monitor_data(&mut self) -> Result<MonitorData, KMBoxNetConnectionError> {
        self.socket
            .recv_from(unsafe {
                std::slice::from_raw_parts_mut(
                    self.monitor.as_mut_ptr() as *mut _,
                    std::mem::size_of::<MonitorData>(),
                )
            })
            .map_err(KMBoxNetConnectionError)?;
        Ok(unsafe { self.monitor.assume_init() })
    }
}

impl KMBoxNet {
    pub fn new(config: KMBoxNetConfig) -> Result<Self, KMBoxNetConnectionError> {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).unwrap();
        let socket_addr = SocketAddr::new(config.ip.parse().unwrap(), config.port);
        let rand = rand::random::<u32>();
        let tx = ClientTx {
            head: structs::CmdHead {
                mac: to_hex(&config.uuid, 4),
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

    /// Set the timeout for the socket
    pub fn set_timeout(&mut self, timeout: std::time::Duration) -> Result<(), std::io::Error> {
        self.socket.set_read_timeout(Some(timeout))?;
        self.socket.set_write_timeout(Some(timeout))?;
        debug!("Timeout set to {:?}", timeout);
        Ok(())
    }

    /// Send a keyboard keydown event
    pub fn keyboard_keydown(&mut self, key: KeyboardKey) -> Result<(), KMBoxNetSendError> {
        let tx = unsafe { self.tx.assume_init_mut() };
        let key = key as u8;
        for i in 0..10 {
            unsafe {
                if tx.data.cmd_keyboard.button[i] == 0 as char {
                    tx.data.cmd_keyboard.button[i] = key as char;
                }
            }
        }
        debug!("Keyboard key set tx\n{:?}", unsafe { tx.data.cmd_keyboard });
        self.send(CMD::KEYBOARD_ALL)?;
        Ok(())
    }

    /// keybord keyup
    pub fn keyboard_keyup(&mut self, key: KeyboardKey) -> Result<(), KMBoxNetSendError> {
        let tx = unsafe { self.tx.assume_init_mut() };
        let key = key as u8;
        for i in 0..10 {
            unsafe {
                if tx.data.cmd_keyboard.button[i] == key as char {
                    tx.data.cmd_keyboard.button[i] = 0 as char;
                }
            }
        }
        debug!("Keyboard key set tx\n{:?}", unsafe { tx.data.cmd_keyboard });
        self.send(CMD::KEYBOARD_ALL)?;
        Ok(())
    }

    /// mouse left click
    pub fn mouse_left_click(
        &mut self,
        state: impl Into<ButtonState>,
    ) -> Result<(), KMBoxNetSendError> {
        let tx = unsafe { self.tx.assume_init_mut() };
        tx.data.cmd_mouse.button = Into::into(state.into());
        debug!("Mouse left button set tx\n{:?}", unsafe {
            tx.data.cmd_mouse
        });
        self.send(CMD::MOUSE_LEFT)?;
        Ok(())
    }

    /// mouse right click
    pub fn mouse_right_click(
        &mut self,
        state: impl Into<ButtonState>,
    ) -> Result<(), KMBoxNetSendError> {
        let tx = unsafe { self.tx.assume_init_mut() };
        tx.data.cmd_mouse.button = Into::into(state.into());
        debug!("Mouse right button set tx\n{:?}", unsafe {
            tx.data.cmd_mouse
        });
        self.send(CMD::MOUSE_RIGHT)?;
        Ok(())
    }

    /// mouse middle wheel click
    pub fn mouse_middle_click(
        &mut self,
        state: impl Into<ButtonState>,
    ) -> Result<(), KMBoxNetSendError> {
        let tx = unsafe { self.tx.assume_init_mut() };
        tx.data.cmd_mouse.button = Into::into(state.into());
        debug!("Mouse right button set tx\n{:?}", unsafe {
            tx.data.cmd_mouse
        });
        self.send(CMD::MOUSE_MIDDLE)?;
        Ok(())
    }

    /// use the mouse scroll wheel
    pub fn mouse_wheel(&mut self, state: impl Into<MwheelState>) -> Result<(), KMBoxNetSendError> {
        let tx = unsafe { self.tx.assume_init_mut() };
        tx.data.cmd_mouse.wheel = Into::into(state.into());
        debug!("Mouse wheel set tx\n{:?}", unsafe { tx.data.cmd_mouse });
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

    /// Monitor the KMBoxNet
    /// This will return a new KMBoxNet instance that can be used to monitor the KMBoxNet
    /// This is useful for getting the current state of the KMBoxNet attached devices
    pub fn into_monitor(mut self) -> Result<KMBoxNetMonitor, KMBoxNetSendError> {
        debug!("Monitor KMBoxNet");
        let tx = unsafe { self.tx.assume_init_mut() };
        tx.head.indexpts += 1;
        tx.head.cmd = CMD::MONITOR.into();
        tx.head.rand = self.socket_addr.port() as u32 + 1_u32 | 0xaa55_u32 << 16_u32;
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
        Ok(KMBoxNetMonitor::new(self.socket_addr))
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
            match cmd {
                CMD::MOUSE_MOVE
                | CMD::MOUSE_LEFT
                | CMD::MOUSE_MIDDLE
                | CMD::MOUSE_RIGHT
                | CMD::MOUSE_WHEEL
                | CMD::MOUSE_AUTOMOVE => {
                    let CmdData { cmd_mouse } = tx.data;
                    {
                        debug!("Send Mouse data tx.data\n{:?}", cmd_mouse);
                    }
                }
                CMD::KEYBOARD_ALL => {
                    let CmdData { cmd_keyboard } = tx.data;
                    {
                        debug!("Send Keyboard data tx.data\n{:?}", cmd_keyboard);
                    }
                }
                CMD::CONNECT => {} // no logging needed
                CMD::REBOOT => {}  // no logging needed
                CMD::BAZER_MOVE => unimplemented!("bazer move not implemented"),
                CMD::MONITOR => unimplemented!("monitor not implemented"),
                CMD::DEBUG => unimplemented!("debug not implemented"),
                CMD::MASK_MOUSE => unimplemented!("mask mouse not implemented"),
                CMD::UNMASK_ALL => unimplemented!("unmask all not implemented"),
                CMD::SETCONFIG => unimplemented!("setconfig not implemented"),
                CMD::SHOWPIC => unimplemented!("showpic not implemented"),
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

impl InputMiddlewareDeviceAction for KMBoxNet {
    fn keyboard_keydown(
        &mut self,
        key: KeyboardKey,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        self.keyboard_keydown(key).map_err(|e| e.into())
    }

    fn keyboard_keyup(
        &mut self,
        key: KeyboardKey,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        self.keyboard_keyup(key).map_err(|e| e.into())
    }

    fn mouse_left_click(
        &mut self,
        state: ButtonState,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        self.mouse_left_click(state).map_err(|e| e.into())
    }

    fn mouse_right_click(
        &mut self,
        state: ButtonState,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        self.mouse_right_click(state).map_err(|e| e.into())
    }

    fn mouse_middle_click(
        &mut self,
        state: ButtonState,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        self.mouse_middle_click(state).map_err(|e| e.into())
    }

    fn mouse_side1_click(
        &mut self,
        _: ButtonState,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        unimplemented!("not possible on kmbox net")
    }

    fn mouse_side2_click(
        &mut self,
        _: ButtonState,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        unimplemented!("not possible on kmbox net")
    }

    fn mouse_wheel_click(
        &mut self,
        state: ButtonState,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        self.mouse_middle_click(state).map_err(|e| e.into())
    }

    fn mouse_wheel(
        &mut self,
        state: MwheelState,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        self.mouse_wheel(state).map_err(|e| e.into())
    }

    fn mouse_move(&mut self, pos: [i32; 2]) -> Result<(), crate::errors::InputMiddlewareSendError> {
        self.mouse_move(pos).map_err(|e| e.into())
    }
}
