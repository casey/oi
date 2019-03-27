use oi::ErrAt;
use std::path::{Path, PathBuf};

fn main() {
  let path = Path::new("Configuration.toml");
  let result = std::fs::File::open(path);
  println!("io::Error:");
  println!("{}", result.as_ref().unwrap_err());
  println!();
  println!("oi::Error w/Path:");
  println!("{}", result.err_at::<PathBuf, _>(path).unwrap_err());
}
