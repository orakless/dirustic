use std::string::FromUtf8Error;
use crate::StdError;

pub enum Error {
    StandardError { e: String },
    NotAPlaylist,
    NotFound,
    YtDlpNotFound,
    YtDlpError { e: String },
    UnknownError { e: String },
    SerenityError { e: String },
}

impl From<StdError> for Error {
    fn from(e: StdError) -> Self {
        Error::StandardError { e: e.to_string() }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error::StandardError { e: e.to_string() }
    }
}

impl From<serenity::Error> for Error {
    fn from(e: serenity::Error) -> Self {
        Error::SerenityError { e: e.to_string() }
    }
}