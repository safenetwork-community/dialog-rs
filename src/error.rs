// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

use std::fmt;
use std::io;
use std::process;
use std::result;
use std::str;
use std::string;

/// A result returned by `dialog`.
pub type Result<T> = result::Result<T, Error>;

/// An error returned by `dialog`.
#[derive(Debug)]
pub enum Error {
    /// A general error with an error message.
    Error(String),
    /// An input or output error.
    IoError(io::Error),
    /// An UTF-8 error.
    Utf8Error(str::Utf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Error(ref s) => write!(f, "Error: {}", s),
            Error::IoError(ref e) => write!(f, "I/O error: {}", e),
            Error::Utf8Error(ref e) => write!(f, "UTF-8 error: {}", e),
        }
    }
}

impl From<&str> for Error {
    fn from(string: &str) -> Error {
        Error::Error(string.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IoError(error)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(error: str::Utf8Error) -> Error {
        Error::Utf8Error(error)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(error: string::FromUtf8Error) -> Error {
        Error::Utf8Error(error.utf8_error())
    }
}

impl From<(&str, process::ExitStatus)> for Error {
    fn from(data: (&str, process::ExitStatus)) -> Error {
        let (command, status) = data;
        let msg = match status.code() {
            Some(code) => format!("Command {} failed with exit status {}", command, code),
            None => format!("Command {} was terminated by a signal", command),
        };
        Error::Error(msg)
    }
}
