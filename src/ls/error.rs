use std::{error::Error, ffi::OsString, fmt, io};

/// The error type for listing MP3 operations.
#[derive(Debug)]
pub enum LsError {
    /// The specified path was invalid.
    InvalidPath(OsString),

    /// An MP3 file was unable to be read or parsed.
    Id3Error(OsString, id3::Error),

    /// The current working directory was unable to be read.
    IoCwdError(io::Error),

    /// A file was unable to be read.
    IoReadError(OsString, io::Error),
}

impl fmt::Display for LsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LsError::InvalidPath(path) => format!("cannot access {:?}: no such file or directory", path),
                LsError::Id3Error(file, err) => format!(
                    "attempting to read {:?} resulted in an error: {}",
                    file,
                    match err.kind {
                        id3::ErrorKind::Io(ref err) => format!("{}", err),
                        _ => format!("{}", err),
                    }
                ),
                LsError::IoCwdError(err) => format!(
                    "attempting to get current working directory resulted in an error: {}",
                    err,
                ),
                LsError::IoReadError(file, err) =>
                    format!("attempting to read {:?} resulted in an error: {}", file, err),
            }
        )
    }
}

impl Error for LsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            LsError::InvalidPath(_) => None,
            LsError::Id3Error(_, ref err) => match err.kind {
                id3::ErrorKind::Io(ref err) => Some(err),
                _ => Some(err),
            },
            LsError::IoCwdError(ref err) => Some(err),
            LsError::IoReadError(_, ref err) => Some(err),
        }
    }
}
