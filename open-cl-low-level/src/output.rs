
use crate::status_code::StatusCodeError;

use anyhow::Result;

pub type Output<T> = Result<T>;

#[inline]
pub fn build_output<T>(payload: T, status_code: i32) -> Output<T> {
    match StatusCodeError::new(status_code) {
        None => Ok(payload),
        Some(e) => Err(e)?,
    }
}
