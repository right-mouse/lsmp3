use std::{error::Error, ffi::OsString, fmt, io};

/// The error type for listing MP3 operations.
#[derive(Debug)]
pub enum LsError {
    /// The specified path was invalid.
    InvalidPath(OsString),

    /// A file was unable to be read.
    IoReadError(OsString, io::Error),

    /// An MP3 file was unable to be read or parsed.
    Id3Error(OsString, id3::Error),
}

impl fmt::Display for LsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LsError::InvalidPath(path) => format!("cannot access {:?}: no such file or directory", path),
                LsError::IoReadError(file, err) =>
                    format!("attempting to read {:?} resulted in an error: {}", file, err),
                LsError::Id3Error(file, err) => format!(
                    "attempting to read {:?} resulted in an error: {}",
                    file,
                    match err.kind {
                        id3::ErrorKind::Io(ref err) => format!("{}", err),
                        _ => format!("{}", err),
                    }
                ),
            }
        )
    }
}

impl Error for LsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            LsError::InvalidPath(_) => None,
            LsError::IoReadError(_, ref err) => Some(err),
            LsError::Id3Error(_, ref err) => match err.kind {
                id3::ErrorKind::Io(ref err) => Some(err),
                _ => Some(err),
            },
        }
    }
}
