use std::fmt::{self, write};
use reqwest::Error as ReqwestError;
use toml::de::Error as TomlError;
use std::io::Error as IOError;

#[derive(Debug)]
pub enum BeetleError {
    HTTP(ReqwestError),
    TOML_ERROR(TomlError),
    IO(IOError),
    MissingValue(String),
    FileNotFound(String),
}

impl fmt::Display for BeetleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BeetleError::HTTP(err) => write!(f, "HTTP error: {}", err),
            BeetleError::TOML_ERROR(err) => write!(f, "TOML error: {}", err),
            BeetleError::IO(err) => write!(f, "IO error: {}", err),
            BeetleError::MissingValue(message) => write!(f, "Missing value: {}", message),
            BeetleError::FileNotFound(message) => write!(f, "File not Found {}", message),
        }
    }
}

impl std::error::Error for BeetleError {}

impl From<ReqwestError> for BeetleError {
    fn from(err: ReqwestError) -> BeetleError {
        BeetleError::HTTP(err)
    }
}

impl From<TomlError> for BeetleError {
    fn from(err: TomlError) -> BeetleError {
        BeetleError::TOML_ERROR(err) 
    }
}

impl From<IOError> for BeetleError {
    fn from(err: IOError) -> BeetleError {
        BeetleError::IO(err)
    }
}