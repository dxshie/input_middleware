use enigo::{Axis, Direction, Key, Keyboard, Mouse, Settings};
use errors::{SysCallConnectionError, SysCallSendError};
use log::debug;

use crate::{button_state::MwheelState, InputMiddlewareDeviceAction};

pub mod errors;

#[derive(Debug, Clone)]
pub struct SysCallConfig;

pub struct SysCall(enigo::Enigo);

unsafe impl Send for SysCall {}
unsafe impl Sync for SysCall {}

impl SysCall {
    pub fn new(_config: SysCallConfig) -> Result<SysCall, SysCallConnectionError> {
        Ok(Self(enigo::Enigo::new(&Settings::default()).unwrap()))
    }
}

impl InputMiddlewareDeviceAction for SysCall {
    fn keyboard_keydown(
        &mut self,
        _key: crate::keyboardkeys::KeyboardKey,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        debug!("not implemented");
        Ok(())
    }

    fn keyboard_keyup(
        &mut self,
        _key: crate::keyboardkeys::KeyboardKey,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        debug!("not implemented");
        Ok(())
    }

    fn mouse_left_click(
        &mut self,
        state: crate::button_state::ButtonState,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        match state {
            crate::button_state::ButtonState::Pressed => {
                self.0
                    .button(enigo::Button::Left, Direction::Press)
                    .map_err(SysCallSendError)?;
            }
            crate::button_state::ButtonState::Released => {
                self.0
                    .button(enigo::Button::Left, Direction::Release)
                    .map_err(SysCallSendError)?;
            }
        }
        Ok(())
    }

    fn mouse_right_click(
        &mut self,
        state: crate::button_state::ButtonState,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        match state {
            crate::button_state::ButtonState::Pressed => {
                self.0
                    .button(enigo::Button::Right, Direction::Press)
                    .map_err(SysCallSendError)?;
            }
            crate::button_state::ButtonState::Released => {
                self.0
                    .button(enigo::Button::Right, Direction::Release)
                    .map_err(SysCallSendError)?;
            }
        }
        Ok(())
    }

    fn mouse_middle_click(
        &mut self,
        state: crate::button_state::ButtonState,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        match state {
            crate::button_state::ButtonState::Pressed => {
                self.0
                    .button(enigo::Button::Middle, Direction::Press)
                    .map_err(SysCallSendError)?;
            }
            crate::button_state::ButtonState::Released => {
                self.0
                    .button(enigo::Button::Middle, Direction::Release)
                    .map_err(SysCallSendError)?;
            }
        }
        Ok(())
    }

    fn mouse_side1_click(
        &mut self,
        state: crate::button_state::ButtonState,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        match state {
            crate::button_state::ButtonState::Pressed => {
                self.0
                    .button(enigo::Button::Back, Direction::Press)
                    .map_err(SysCallSendError)?;
            }
            crate::button_state::ButtonState::Released => {
                self.0
                    .button(enigo::Button::Back, Direction::Release)
                    .map_err(SysCallSendError)?;
            }
        }
        Ok(())
    }

    fn mouse_side2_click(
        &mut self,
        state: crate::button_state::ButtonState,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        match state {
            crate::button_state::ButtonState::Pressed => {
                self.0
                    .button(enigo::Button::Forward, Direction::Press)
                    .map_err(SysCallSendError)?;
            }
            crate::button_state::ButtonState::Released => {
                self.0
                    .button(enigo::Button::Forward, Direction::Release)
                    .map_err(SysCallSendError)?;
            }
        }
        Ok(())
    }

    fn mouse_wheel_click(
        &mut self,
        state: crate::button_state::ButtonState,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        match state {
            crate::button_state::ButtonState::Pressed => {
                self.0
                    .button(enigo::Button::Middle, Direction::Press)
                    .map_err(SysCallSendError)?;
            }
            crate::button_state::ButtonState::Released => {
                self.0
                    .button(enigo::Button::Middle, Direction::Release)
                    .map_err(SysCallSendError)?;
            }
        }
        Ok(())
    }

    fn mouse_wheel(
        &mut self,
        state: crate::button_state::MwheelState,
    ) -> Result<(), crate::errors::InputMiddlewareSendError> {
        match state {
            MwheelState::Up(data) => {
                self.0
                    .scroll(data, Axis::Vertical)
                    .map_err(SysCallSendError)?;
            }
            MwheelState::Down(data) => {
                self.0
                    .scroll(-data, Axis::Vertical)
                    .map_err(SysCallSendError)?;
            }
            MwheelState::Released => {
                self.0.scroll(0, Axis::Vertical).map_err(SysCallSendError)?;
            }
        }
        Ok(())
    }

    fn mouse_move(&mut self, pos: [i32; 2]) -> Result<(), crate::errors::InputMiddlewareSendError> {
        self.0
            .move_mouse(pos[0], pos[1], enigo::Coordinate::Rel)
            .map_err(SysCallSendError)?;
        Ok(())
    }
}
