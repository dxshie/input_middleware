use thiserror::Error;

use crate::errors::InputMiddlewareSendError;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct KMBoxNetConnectionError(#[from] pub std::io::Error);

#[derive(Error, Debug)]
#[error(transparent)]
pub struct KMBoxNetSendError(#[from] pub std::io::Error);

impl From<KMBoxNetSendError> for InputMiddlewareSendError {
    fn from(e: KMBoxNetSendError) -> Self {
        InputMiddlewareSendError(e.0)
    }
}
