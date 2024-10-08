use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, Deserialize, Serialize)]
pub enum QMPInputSendKeyType {
    #[default]
    Key,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum QMPInputSendKeyDataType {
    QCode,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum QMPSendKeyCode {
    Ctrl,
    Alt,
    Delete,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct QMPInputSendKeyData {
    pub down: bool,
    pub key: QMPSendKeyCodeData,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct QMPSendKeyCodeData {
    #[serde(rename(serialize = "type"))]
    send_type: QMPInputSendKeyDataType,
    data: QMPSendKeyCode,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct QMPInputSendKeyEvent {
    #[serde(rename(serialize = "type"))]
    send_type: QMPInputSendKeyType,
    data: QMPInputSendKeyData,
}
