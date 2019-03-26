use std::path::Path;

use oi::ErrAt;

fn main() {
  let path = Path::new("Configuration.toml");
  let result = std::fs::File::open(path);
  println!("io::Error:");
  println!("{}", result.as_ref().unwrap_err());
  println!();
  println!("oi::Error:");
  println!("{}", result.err_at(path).unwrap_err());
}
