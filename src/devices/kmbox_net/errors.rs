use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct KMBoxNetConnectionError(#[from] pub std::io::Error);

#[derive(Error, Debug)]
#[error(transparent)]
pub struct KMBoxNetSendError(#[from] pub std::io::Error);
