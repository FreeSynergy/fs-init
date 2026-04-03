//! Error type for fs-init.

use std::fmt;

/// All errors that can occur during bootstrap.
#[derive(Debug)]
pub enum FsInitError {
    /// Store clone failed.
    StoreClone(String),
    /// I/O error (stdin/stdout).
    Io(std::io::Error),
    /// User aborted the wizard.
    Aborted,
    /// Install pipeline failed.
    Install(String),
}

impl fmt::Display for FsInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FsInitError::StoreClone(msg) => write!(f, "Store clone failed: {msg}"),
            FsInitError::Io(e) => write!(f, "I/O error: {e}"),
            FsInitError::Aborted => write!(f, "Aborted by user"),
            FsInitError::Install(msg) => write!(f, "Install failed: {msg}"),
        }
    }
}

impl std::error::Error for FsInitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FsInitError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for FsInitError {
    fn from(e: std::io::Error) -> Self {
        FsInitError::Io(e)
    }
}
