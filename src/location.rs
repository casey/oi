use std::{
  fmt::{self, Display, Formatter},
  net::SocketAddr,
  path::{Path, PathBuf},
  process::Command,
};

/// Location where an io::Error might occur
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Location {
  /// Path to a file
  File(PathBuf),
  /// Unnamed temporary file
  Tempfile,
  /// Terminal
  Terminal,
  /// Standard input
  Stdin,
  /// Standard output
  Stdout,
  /// Standard error
  Stderr,
  /// Network address
  Address(SocketAddr),
  /// Command
  Command(String),
  /// Something else
  Other(String),
}

impl From<PathBuf> for Location {
  fn from(path: PathBuf) -> Location {
    Location::File(path)
  }
}

impl From<&Path> for Location {
  fn from(path: &Path) -> Location {
    Location::File(path.to_owned())
  }
}

impl From<SocketAddr> for Location {
  fn from(address: SocketAddr) -> Location {
    Location::Address(address)
  }
}

impl From<&Command> for Location {
  fn from(command: &Command) -> Location {
    Location::Command(format!("{:?}", command))
  }
}

impl Display for Location {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    use Location::*;
    match self {
      File(path) => write!(f, "{}", path.display()),
      Tempfile => write!(f, "temporary file"),
      Terminal => write!(f, "terminal"),
      Stdin => write!(f, "stdin"),
      Stdout => write!(f, "stdout"),
      Stderr => write!(f, "stderr"),
      Address(address) => write!(f, "{}", address),
      Other(other) => write!(f, "{}", other),
      Command(command) => write!(f, "{}", command),
    }
  }
}
