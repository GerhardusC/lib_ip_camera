use std::convert::Infallible;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    // #[error("Invalid timestamp")]
    // InvalidTimestampRequest(SystemTimeError),
    #[error("IO Error.")] //Name error.
    IoError(std::io::Error), //Add optional errors inside.
    #[error("Failed to write logs to file.")]
    LogWriterError,
    #[error("Infallible.")]
    Infallible,

    #[error("Failed to read IP address or port.")]
    IPError,

    #[error("No path supplied to log to.")]
    NoPathSuppliedToLog,

    #[error("Something really weird must have happened for you to end up in this position, damn.")]
    WhatTheFunkError,
}

// Example to impl from:
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}

impl From<Infallible> for Error {
    fn from(_value: Infallible) -> Self {
        Error::Infallible
    }
}
