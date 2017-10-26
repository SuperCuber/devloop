use std::fmt::{Display, Formatter, Error as FmtError};
use std::io::Error as IoError;

use yaml_rust::ScanError;

pub enum DevloopError {
    Io(IoError),
    Scan(ScanError),
    InvalidConfig,
}

impl From<IoError> for DevloopError {
    fn from(other: IoError) -> Self {
        DevloopError::Io(other)
    }
}

impl From<ScanError> for DevloopError {
    fn from(other: ScanError) -> Self {
        DevloopError::Scan(other)
    }
}

impl Display for DevloopError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        match *self {
            DevloopError::Io(ref error) => error.fmt(fmt),
            DevloopError::Scan(ref error) => error.fmt(fmt),
            DevloopError::InvalidConfig => "Invalid config".fmt(fmt),
        }
    }
}
