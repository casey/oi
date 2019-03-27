fn usage() -> Result<(), Error<io::Error, PathBuf>> {
  let p = Path::new("hello");

  let r: Result<File, Error<io::Error, PathBuf>> = std::fs::File::open(p).err_at(p);

  let pb = p.to_path_buf();

  let r: Result<File, Error<io::Error, PathBuf>> = std::fs::File::open(p).err_at(pb);

  let pb = p.to_path_buf();

  let r: Result<File, Error<io::Error, PathBuf>> = std::fs::File::open(p).err_at(&pb);

  let pb = p.to_path_buf();

  let r: Result<File, Error<io::Error, PathBuf>> = std::fs::File::open(p).err_at(&&&pb);

  let s = "hello";

  std::fs::File::open(p).err_at(s)?;

  Ok(())
}
