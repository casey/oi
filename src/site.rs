use crate::Location;

use failure::Fail;

use std::{
  fmt::{self, Formatter},
  net::SocketAddr,
  path::{Path, PathBuf},
  process::Command,
};

#[derive(Debug)]
pub enum Site {
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
  /// Network host
  Host(String),
  /// Network host and address pair
  ResolvedHost(String, SocketAddr),
  /// Command
  Command(String, Vec<String>),
  /// Something else
  Other(String),
}

impl Location for Site {
  fn fmt_error(&self, f: &mut Formatter, error: &dyn Fail) -> fmt::Result {
    use Site::*;
    match self {
      File(path) => write!(f, "{}", path.display())?,
      Tempfile => write!(f, "temporary file")?,
      Terminal => write!(f, "terminal")?,
      Stdin => write!(f, "standard input")?,
      Stdout => write!(f, "standard output")?,
      Stderr => write!(f, "standard error")?,
      Address(address) => write!(f, "{}", address)?,
      Host(host) => write!(f, "{}", host)?,
      ResolvedHost(host, address) => write!(f, "{} ({})", host, address)?,
      Command(command, args) => {
        if args.is_empty() {
          write!(f, "{}", command)?;
        } else {
          write!(f, "{} {}", command, args.join(" "))?;
        }
      }
      Other(other) => write!(f, "{}", other)?,
    }

    write!(f, ": {}", error)
  }
}

impl From<&Command> for Site {
  fn from(command: &Command) -> Site {
    Site::Command(format!("{:?}", command), Vec::new())
  }
}

impl From<&Path> for Site {
  fn from(path: &Path) -> Site {
    Site::File(path.to_owned())
  }
}

impl From<PathBuf> for Site {
  fn from(path: PathBuf) -> Site {
    Site::File(path)
  }
}

impl From<SocketAddr> for Site {
  fn from(address: SocketAddr) -> Site {
    Site::Address(address)
  }
}
