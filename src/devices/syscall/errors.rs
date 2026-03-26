use std::io::Error;

use enigo::InputError;
use thiserror::Error;

use crate::errors::InputMiddlewareSendError;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct SysCallConnectionError(#[from] pub std::io::Error);

#[derive(Error, Debug)]
#[error(transparent)]
pub struct SysCallSendError(#[from] pub InputError);

impl From<SysCallSendError> for InputMiddlewareSendError {
    fn from(e: SysCallSendError) -> Self {
        match e.0 {
            InputError::Mapping(e) => Self(Error::new(std::io::ErrorKind::Unsupported, e)),
            InputError::Unmapping(e) => Self(Error::new(std::io::ErrorKind::Unsupported, e)),
            InputError::NoEmptyKeycodes => Self(Error::new(std::io::ErrorKind::NotFound, e)),
            InputError::Simulate(e) => Self(Error::new(std::io::ErrorKind::ConnectionRefused, e)),
            InputError::InvalidInput(e) => Self(Error::new(std::io::ErrorKind::InvalidInput, e)),
        }
    }
}
