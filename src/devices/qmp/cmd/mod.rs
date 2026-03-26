pub mod key;
pub mod mouse;

use key::QMPInputSendKeyEvent;
use mouse::{QMPInputSendMouseEvent, QMPInputSendMouseMoveData, QMPInputSendMouseType};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ExecuteEvent {
    #[default]
    #[serde(rename(serialize = "input-send-event"))]
    InputSendEvent,
    #[serde(rename(serialize = "qmp_capabilities"))]
    QMPCapabilities,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QMPInputSendArgs<T> {
    events: Vec<T>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QMPMessage<T> {
    execute: ExecuteEvent,
    arguments: Option<QMPInputSendArgs<T>>,
}

impl QMPMessage<QMPInputSendMouseEvent> {
    pub fn new(data: [QMPInputSendMouseMoveData; 2]) -> Self {
        Self {
            execute: ExecuteEvent::InputSendEvent,
            arguments: Some(QMPInputSendArgs {
                events: vec![
                    QMPInputSendMouseEvent {
                        send_type: QMPInputSendMouseType::Rel,
                        data: data[0],
                    },
                    QMPInputSendMouseEvent {
                        send_type: QMPInputSendMouseType::Rel,
                        data: data[1],
                    },
                ],
            }),
        }
    }
}

impl QMPMessage<QMPInputSendKeyEvent> {
    pub fn new(data: Vec<QMPInputSendKeyEvent>) -> Self {
        Self {
            execute: ExecuteEvent::InputSendEvent,
            arguments: Some(QMPInputSendArgs { events: data }),
        }
    }
}

impl<T> QMPMessage<T> {
    pub fn auth() -> Self {
        Self {
            execute: ExecuteEvent::QMPCapabilities,
            arguments: None,
        }
    }
}
