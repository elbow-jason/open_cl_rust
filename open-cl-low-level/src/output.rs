use crate::error::Error;
use crate::status_code::StatusCodeError;

pub type Output<T> = Result<T, Error>;

#[inline]
pub fn build_output<T>(payload: T, status_code: i32) -> Output<T> {
    match StatusCodeError::new(status_code) {
        None => Ok(payload),
        Some(status_code_error) => Err(Error::from(status_code_error)),
    }
}
