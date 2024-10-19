use std::fmt;
use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub enum BeetleError {
    HTTP(ReqwestError),
    MissingValue(String),
    FILE_NOT_FOUND(String),
}

impl fmt::Display for BeetleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BeetleError::HTTP(err) => write!(f, "HTTP error: {}", err),
            BeetleError::MissingValue(message) => write!(f, "Missing value: {}", message),
            BeetleError::FILE_NOT_FOUND(message) => write!(f, "File not Found {}", message),
        }
    }
}

impl std::error::Error for BeetleError {}

impl From<ReqwestError> for BeetleError {
    fn from(err: ReqwestError) -> BeetleError {
        BeetleError::HTTP(err)
    }
}