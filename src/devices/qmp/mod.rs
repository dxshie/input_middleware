pub mod cmd;
pub mod errors;

use std::{io::Write, net::TcpStream};

use cmd::{
    mouse::{QMPInputSendMouseEvent, QMPInputSendMouseMoveData},
    QMPMessage,
};
use errors::QMPConnectionError;
use log::debug;

use crate::{errors::InputMiddlewareSendError, InputMiddlewareDeviceAction};

#[derive(Debug, Clone)]
pub struct QMPConfig {
    pub host: String,
    pub port: u16,
}

pub struct QMPConnection {
    stream: TcpStream,
}

impl QMPConnection {
    pub fn new(config: QMPConfig) -> Result<QMPConnection, QMPConnectionError> {
        let stream = TcpStream::connect(format!("{}:{}", config.host, config.port))?;
        Ok(Self { stream })
    }
}

impl InputMiddlewareDeviceAction for QMPConnection {
    fn keyboard_keydown(
        &mut self,
        _key: crate::keyboardkeys::KeyboardKey,
    ) -> Result<(), InputMiddlewareSendError> {
        todo!()
    }

    fn keyboard_keyup(
        &mut self,
        _key: crate::keyboardkeys::KeyboardKey,
    ) -> Result<(), InputMiddlewareSendError> {
        todo!()
    }

    fn mouse_left_click(
        &mut self,
        _state: crate::button_state::ButtonState,
    ) -> Result<(), InputMiddlewareSendError> {
        todo!()
    }

    fn mouse_right_click(
        &mut self,
        _state: crate::button_state::ButtonState,
    ) -> Result<(), InputMiddlewareSendError> {
        todo!()
    }

    fn mouse_middle_click(
        &mut self,
        _state: crate::button_state::ButtonState,
    ) -> Result<(), InputMiddlewareSendError> {
        todo!()
    }

    fn mouse_side1_click(
        &mut self,
        _state: crate::button_state::ButtonState,
    ) -> Result<(), InputMiddlewareSendError> {
        todo!()
    }

    fn mouse_side2_click(
        &mut self,
        _state: crate::button_state::ButtonState,
    ) -> Result<(), InputMiddlewareSendError> {
        todo!()
    }

    fn mouse_wheel_click(
        &mut self,
        _state: crate::button_state::ButtonState,
    ) -> Result<(), InputMiddlewareSendError> {
        todo!()
    }

    fn mouse_wheel(
        &mut self,
        _state: crate::button_state::MwheelState,
    ) -> Result<(), InputMiddlewareSendError> {
        todo!()
    }

    fn mouse_move(&mut self, pos: [i32; 2]) -> Result<(), InputMiddlewareSendError> {
        let msg =
            QMPMessage::<QMPInputSendMouseEvent>::new(QMPInputSendMouseMoveData::from_slice(&pos));
        let serialized =
            serde_json::to_vec(&msg).map_err(|e| InputMiddlewareSendError(e.into()))?;
        debug!("Sending mouse move to QMP: {:#?}", &msg);
        self.stream.write(&*serialized)?;
        self.stream.flush()?;
        Ok(())
    }
}
