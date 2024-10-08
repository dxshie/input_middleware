//! # Input Middlware
//!
//! ## Synopsis
//! Input Middlware is a crate that implements various hardware devices that act as a middleman between input devices and the computer.
//! Each device may have it's own way of connecting and communicating which is abstracted away in this crate.
//!
//! # Supported Devices
//!
//! - ✅ KMBox_Net (SOCKET)
//! - ❌ KMBox_B (COM) (i don't own that one feel free to do a PR)
//! - ... open a issue if you want more support or create a PR
//!
//! # Example
//! ```rust
//!
//! use input_middleware::devices::kmbox_net::{KMBoxNet, KMBoxNetConfig};
//! use input_middleware::{InputDevice, InputMiddleware};
//!    
//! let config = KMBoxNetConfig::default_with_uuid(env!("KMBOX_UUID"));
//! let km = KMBoxNet::new(config);
//! match km {
//!   Ok(mut km) => {
//!     km.mouse_move([1, 1]);
//!     // ...
//!   }
//!   Err(_) => println!("Failed to connect to KMBox Net"),
//! }
//! ```

use button_state::{ButtonState, MwheelState};
use devices::qmp::{QMPConfig, QMPConnection};
use devices::syscall::SysCallConfig;
use devices::{kmbox_net::KMBoxNetConfig, syscall::SysCall};
use errors::{InputMiddlewareConnectionError, InputMiddlewareSendError};
use keyboardkeys::KeyboardKey;

pub mod button_state;
pub mod devices;
pub mod errors;
pub mod keyboardkeys;
use devices::kmbox_net::KMBoxNet;

#[derive(Debug, Clone)]
pub enum InputDevice {
    KMBoxNet(KMBoxNetConfig),
    SysCall(SysCallConfig),
    QMP(QMPConfig),
}

pub struct InputMiddleware;

impl InputMiddleware {
    pub fn new(
        device: InputDevice,
    ) -> Result<Box<dyn InputMiddlewareDeviceAction>, InputMiddlewareConnectionError> {
        match device {
            InputDevice::KMBoxNet(config) => {
                let km = KMBoxNet::new(config.clone())
                    .map_err(|e| InputMiddlewareConnectionError(e.0))?;
                return Ok(Box::new(km));
            }
            InputDevice::SysCall(config) => {
                let syscall = SysCall::new(config.clone())
                    .map_err(|e| InputMiddlewareConnectionError(e.0))?;
                return Ok(Box::new(syscall));
            }
            InputDevice::QMP(config) => {
                let qmp = QMPConnection::new(config.clone())
                    .map_err(|e| InputMiddlewareConnectionError(e.0))?;
                return Ok(Box::new(qmp));
            }
        }
    }
}

/// The InputMiddlewareDeviceAction trait is used to define the actions that can be performed on an input device.
pub trait InputMiddlewareDeviceAction {
    fn keyboard_keydown(&mut self, key: KeyboardKey) -> Result<(), InputMiddlewareSendError>;
    fn keyboard_keyup(&mut self, key: KeyboardKey) -> Result<(), InputMiddlewareSendError>;
    fn mouse_left_click(&mut self, state: ButtonState) -> Result<(), InputMiddlewareSendError>;
    fn mouse_right_click(&mut self, state: ButtonState) -> Result<(), InputMiddlewareSendError>;
    fn mouse_middle_click(&mut self, state: ButtonState) -> Result<(), InputMiddlewareSendError>;
    fn mouse_side1_click(&mut self, state: ButtonState) -> Result<(), InputMiddlewareSendError>;
    fn mouse_side2_click(&mut self, state: ButtonState) -> Result<(), InputMiddlewareSendError>;
    fn mouse_wheel_click(&mut self, state: ButtonState) -> Result<(), InputMiddlewareSendError>;
    fn mouse_wheel(&mut self, state: MwheelState) -> Result<(), InputMiddlewareSendError>;
    fn mouse_move(&mut self, pos: [i32; 2]) -> Result<(), InputMiddlewareSendError>;
}
