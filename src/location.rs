use std::{
  fmt::{self, Debug, Formatter},
  net::SocketAddr,
  path::PathBuf,
};

use failure::Fail;

/// Location that an `Error` can occur
pub trait Location: Debug + Send + Sync + 'static {
  /// Format the location and error for display
  fn fmt_error(&self, f: &mut Formatter, error: &dyn Fail) -> fmt::Result;
}

impl Location for PathBuf {
  fn fmt_error(&self, f: &mut Formatter, error: &dyn Fail) -> fmt::Result {
    write!(f, "{}: {}", self.display(), error)
  }
}

impl Location for SocketAddr {
  fn fmt_error(&self, f: &mut Formatter, error: &dyn Fail) -> fmt::Result {
    write!(f, "{}: {}", self, error)
  }
}

impl Location for String {
  fn fmt_error(&self, f: &mut Formatter, error: &dyn Fail) -> fmt::Result {
    write!(f, "{}: {}", self, error)
  }
}
