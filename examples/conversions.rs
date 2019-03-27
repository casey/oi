use std::{
  io,
  path::{Path, PathBuf},
};

use oi::ErrAt;

fn main() -> oi::Result<(), io::Error, PathBuf> {
  // Any type that can be converted to the location type can be used:
  let p = Path::new("hello");

  std::fs::File::open(p).err_at(p)?;

  let pb = p.to_path_buf();

  std::fs::File::open(p).err_at(pb)?;

  let pb = p.to_path_buf();

  std::fs::File::open(p).err_at(&pb)?;

  let pb = p.to_path_buf();

  std::fs::File::open(p).err_at(&&&pb)?;

  let s = "hello";

  std::fs::File::open(p).err_at(s)?;

  let s = "hello".to_string();

  std::fs::File::open(p).err_at(s)?;

  Ok(())
}
