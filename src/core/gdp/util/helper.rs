use crate::errors::beetle_error::BeetleError;


pub fn extract_value<T>(opt: Option<T>, error_message: &'static str) -> Result<T, BeetleError> {
    opt.ok_or_else(
        || BeetleError::MissingValue(error_message.to_string()))
}
