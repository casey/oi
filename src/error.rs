use std::{
  error,
  fmt::{self, Display, Formatter},
  io,
};

use crate::location::Location;

/// Location-annotated io::Error with a user-friendly `Display` implementation
#[derive(Debug)]
pub struct Error {
  /// Underlying io::Error
  pub io_error: io::Error,
  /// Location at which the io::Error occurred
  pub location: Location,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.location, self.io_error)
  }
}

impl error::Error for Error {
  fn description(&self) -> &str {
    self.io_error.description()
  }

  #[allow(deprecated)]
  fn cause(&self) -> Option<&dyn error::Error> {
    self.io_error.cause()
  }
}
