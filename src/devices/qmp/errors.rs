use thiserror::Error;

use crate::errors::InputMiddlewareSendError;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct QMPConnectionError(#[from] pub std::io::Error);

#[derive(Error, Debug)]
#[error(transparent)]
pub struct QMPSendError(#[from] pub std::io::Error);

impl From<QMPSendError> for InputMiddlewareSendError {
    fn from(e: QMPSendError) -> Self {
        Self(e.0)
    }
}
