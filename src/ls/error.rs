use glob::{GlobError, PatternError};
use std::{error::Error, ffi::OsString, fmt, io};

#[derive(Debug)]
pub enum LsError {
    InvalidPath(OsString),
    CannotGlobPath(OsString),
    PatternError(PatternError),
    GlobError(GlobError),
    Id3Error(OsString, id3::Error),
    IoCwdError(io::Error),
    IoReadError(OsString, io::Error),
}

impl fmt::Display for LsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LsError::InvalidPath(path) => format!("cannot access {:?}: no such file or directory", path),
                LsError::CannotGlobPath(path) => format!("cannot glob {:?} since it is not a UTF-8 valid path", path),
                LsError::PatternError(err) => format!("glob syntax error: {}", err.msg),
                LsError::GlobError(err) => format!("{}", err),
                LsError::Id3Error(file, err) => format!("attempting to read {:?} resulted in an error: {}", file, err),
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
            LsError::CannotGlobPath(_) => None,
            LsError::PatternError(ref err) => Some(err),
            LsError::GlobError(ref err) => Some(err),
            LsError::Id3Error(_, ref err) => Some(err),
            LsError::IoCwdError(ref err) => Some(err),
            LsError::IoReadError(_, ref err) => Some(err),
        }
    }
}

impl From<PatternError> for LsError {
    fn from(err: PatternError) -> Self {
        LsError::PatternError(err)
    }
}

impl From<GlobError> for LsError {
    fn from(err: GlobError) -> Self {
        LsError::GlobError(err)
    }
}
