//! # Input Middleware
//!
//! ## Synopsis
//!
//! Input Middleware is a crate that implements various input methods that act as a middleman between input devices and the computer.
//! Each method may have it's own way of connecting and communicating which is abstracted away in this crate.
//!
//! # Supported Devices
//!
//! - ✅ Enigo (SysCall) [windows, linux]
//! - ✅ QMP Qemu (TCP Stream)
//! - ✅ KMBox_Net (SOCKET)
//! - ❌ KMBox_B (COM) (i don't own that one feel free to do a PR)
//! - ... open a issue if you want more support or create a PR
//!
//! # Example
//! ```rust
//!
//! use input_middleware::devices::kmbox_net::{KMBoxNet, KMBoxNetConfig};
//! use input_middleware::{InputDevice, InputMiddleware, InputMiddlewareDeviceAction};
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
#[cfg(feature = "kmbox_net")]
use devices::kmbox_net::{KMBoxNet, KMBoxNetConfig};
#[cfg(feature = "qmp")]
use devices::qmp::{QMPConfig, QMPConnection};
#[cfg(feature = "syscall")]
use devices::syscall::{SysCall, SysCallConfig};
use errors::{InputMiddlewareConnectionError, InputMiddlewareSendError};
use keyboardkeys::KeyboardKey;

pub mod button_state;
pub mod devices;
pub mod errors;
pub mod keyboardkeys;

#[derive(Debug, Clone)]
pub enum InputDevice {
    #[cfg(feature = "kmbox_net")]
    KMBoxNet(KMBoxNetConfig),
    #[cfg(feature = "syscall")]
    SysCall(SysCallConfig),
    #[cfg(feature = "qmp")]
    QMP(QMPConfig),
}

/// InputMiddleware is the main struct that is used to create a new input device.
pub struct InputMiddleware;

impl InputMiddleware {
    pub fn new(
        device: InputDevice,
    ) -> Result<Box<dyn InputMiddlewareDeviceAction>, InputMiddlewareConnectionError> {
        match device {
            #[cfg(feature = "kmbox_net")]
            InputDevice::KMBoxNet(config) => {
                let km = KMBoxNet::new(config.clone())
                    .map_err(|e| InputMiddlewareConnectionError(e.0))?;
                return Ok(Box::new(km));
            }
            #[cfg(feature = "syscall")]
            InputDevice::SysCall(config) => {
                let syscall = SysCall::new(config.clone())
                    .map_err(|e| InputMiddlewareConnectionError(e.0))?;
                return Ok(Box::new(syscall));
            }
            #[cfg(feature = "qmp")]
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
    /// Press a key
    fn keyboard_keydown(&mut self, key: KeyboardKey) -> Result<(), InputMiddlewareSendError>;
    /// Release a key
    fn keyboard_keyup(&mut self, key: KeyboardKey) -> Result<(), InputMiddlewareSendError>;
    /// click the mouse left
    fn mouse_left_click(&mut self, state: ButtonState) -> Result<(), InputMiddlewareSendError>;
    /// click the mouse right
    fn mouse_right_click(&mut self, state: ButtonState) -> Result<(), InputMiddlewareSendError>;
    /// click the mouse middle
    fn mouse_middle_click(&mut self, state: ButtonState) -> Result<(), InputMiddlewareSendError>;
    /// click the mouse side1
    fn mouse_side1_click(&mut self, state: ButtonState) -> Result<(), InputMiddlewareSendError>;
    /// click the mouse side2
    fn mouse_side2_click(&mut self, state: ButtonState) -> Result<(), InputMiddlewareSendError>;
    /// click the mouse wheel
    fn mouse_wheel_click(&mut self, state: ButtonState) -> Result<(), InputMiddlewareSendError>;
    /// Move the mouse wheel.
    fn mouse_wheel(&mut self, state: MwheelState) -> Result<(), InputMiddlewareSendError>;
    /// Move the mouse to the given position relative.
    fn mouse_move(&mut self, pos: [i32; 2]) -> Result<(), InputMiddlewareSendError>;
}
