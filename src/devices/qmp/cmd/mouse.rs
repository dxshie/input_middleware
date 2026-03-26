use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, Deserialize, Serialize)]
pub enum QMPInputSendMouseType {
    #[default]
    Rel,
    Abs,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum QMPInputSendAxis {
    X,
    Y,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct QMPInputSendMouseEvent {
    #[serde(rename(serialize = "type"))]
    pub send_type: QMPInputSendMouseType,
    pub data: QMPInputSendMouseMoveData,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct QMPInputSendMouseMoveData {
    axis: QMPInputSendAxis,
    value: i32,
}

impl QMPInputSendMouseMoveData {
    pub fn new(axis: QMPInputSendAxis, value: i32) -> Self {
        Self { axis, value }
    }

    pub fn from_slice(data: &[i32; 2]) -> [Self; 2] {
        [
            Self::new(QMPInputSendAxis::X, data[0]),
            Self::new(QMPInputSendAxis::Y, data[1]),
        ]
    }
}
