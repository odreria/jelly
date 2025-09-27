use crate::errors::{JellyError, Result};

pub fn extract_value<T>(opt: Option<T>, error_message: &'static str) -> Result<T> {
    opt.ok_or_else(|| JellyError::missing_configuration(error_message))
}
