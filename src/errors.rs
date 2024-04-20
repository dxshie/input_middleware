use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct InputMiddlewareConnectionError(#[from] pub std::io::Error);

#[derive(Error, Debug)]
#[error(transparent)]
pub struct InputMiddlewareSendError(#[from] pub std::io::Error);
