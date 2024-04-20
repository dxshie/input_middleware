//! # Input Middleware
//! This library is a collection of input devices and utilities for handling input events.
//!
//! ## Features
//! - KMBoxNet: A networked input device that can send keyboard and mouse events to a KMBoxNet device.
//!
//! ## Usage
//! ```rust
//!
//! use input_middleware::devices::kmbox_net::{KMBoxNet, KMBoxNetConfig};
//! use input_middleware::{InputDevice, InputMiddleware};
//!    
//! let config = KMBoxNetConfig::default_with_uuid("XXXXXXX");
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
use devices::kmbox_net::KMBoxNetConfig;
use errors::{InputMiddlewareConnectionError, InputMiddlewareSendError};
use keyboardkeys::KeyboardKey;

pub mod button_state;
pub mod devices;
pub mod errors;
pub mod keyboardkeys;
use devices::kmbox_net::KMBoxNet;

// The devices that are supported by this library.
// TODO: Add more devices here.
// add layer of abstraction to device initialization
// pass enum to select device
#[derive(Debug, Clone)]
pub enum InputDevice {
    KMBoxNet(KMBoxNetConfig),
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
