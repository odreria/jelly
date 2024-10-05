use std::fmt;
use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub enum BeetleError {
    HTTP(ReqwestError),
    MissingValue(String),
}

impl fmt::Display for BeetleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BeetleError::HTTP(err) => write!(f, "HTTP error: {}", err),
            BeetleError::MissingValue(message) => write!(f, "Missing value: {}", message),
        }
    }
}

impl std::error::Error for BeetleError {}

impl From<ReqwestError> for BeetleError {
    fn from(err: ReqwestError) -> BeetleError {
        BeetleError::HTTP(err)
    }
}