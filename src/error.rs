use std::fmt::{self, Display, Formatter};

use failure::{Backtrace, Fail};

use crate::location::Location;

/// Location-annotated io::Error with a user-friendly `Display` implementation
#[derive(Debug)]
pub struct Error<E: Fail, L: Location> {
  /// Error
  pub error: E,
  /// Location at which `error` occurred
  pub location: L,
}

impl<E: Fail, L: Location> Fail for Error<E, L> {
  fn name(&self) -> Option<&str> {
    self.error.name()
  }

  fn cause(&self) -> Option<&Fail> {
    self.error.cause()
  }

  fn backtrace(&self) -> Option<&Backtrace> {
    self.error.backtrace()
  }
}

impl<E: Fail, L: Location> Display for Error<E, L> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    self.location.fmt_error(f, &self.error)
  }
}
