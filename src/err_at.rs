use std::io;

use crate::{Error, Location, Result};

/// Extension trait for annotating io::Errors
pub trait ErrAt<T> {
  /// Add location if self is an error
  fn err_at<L: Into<Location>>(self, location: L) -> Result<T>;
}

/// Extend io::Result with err_at
impl<T> ErrAt<T> for io::Result<T> {
  fn err_at<L: Into<Location>>(self, location: L) -> Result<T> {
    match self {
      // pass through ok value
      Ok(ok) => Ok(ok),
      // annotate err with location
      Err(io_error) => Err(Error {
        io_error,
        location: location.into(),
      }),
    }
  }
}
