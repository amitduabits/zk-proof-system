//! Error types for the ZK proof system

use std::fmt;

/// Main error type for the ZK proof system
#[derive(Debug)]
pub enum Error {
    /// Circuit synthesis error
    Synthesis(String),
    /// Verification error
    Verification(String),
    /// IO error
    Io(std::io::Error),
    /// Other errors
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Synthesis(msg) => write!(f, "Synthesis error: {msg}"),
            Self::Verification(msg) => write!(f, "Verification error: {msg}"),
            Self::Io(err) => write!(f, "IO error: {err}"),
            Self::Other(msg) => write!(f, "Error: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;
