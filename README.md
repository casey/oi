# oi

[![crates.io](https://img.shields.io/crates/v/oi.svg)](https://crates.io/crates/oi) [![docs](https://docs.rs/oi/badge.svg)](http://docs.rs/oi)

`oi` provides a location-annotated error type so you can display useful
error messages to your users.

Error messages without location information are often not actionable,
especially when they come from a complex program with many potential
sources of errors.

Compare an unannotated error:

```sh
$ foo
No such file or directory (os error 2)
```

…with an annotated error:

```sh
$ foo
Configuration.toml: No such file or directory (os error 2)
```

`oi` is named after the exclamation, as in [Oi! Oi! Oi!](https://youtu.be/XWLU76o5rEI).
Imagine alerting your users to the location of errors: "Oi! 1.2.3.4 is unreachable!"

## usage

This crate provides an `Error` type that wraps an error and location, a `Location`
trait for error locations, an `ErrAt` trait that extends `Result` with an
`err_at` method to annotate err values with locations, and a `Result<T, E, L>`
type as an alias for the more cumbersome `Result<T, Error<E, L>>`:

```rust
pub struct Error<E: Fail, L: Location> {
  pub error: E,
  pub location: L,
}

pub trait Location: Debug + Send + Sync + 'static {
  fn fmt_error(&self, f: &mut Formatter, error: &dyn Fail) -> fmt::Result;
}

pub trait ErrAt<T, E: Fail> {
  fn err_at<L: Location, I: Into<L>>(self, location: I) -> Result<T, Error<E, L>>;
}

pub type Result<T, E, L> = std::result::Result<T, Error<E, L>>;
```

`Location` is implemented for `PathBuf` and `SocketAddr`, and can easily be implemented
for custom location types. The one required method, `fmt_error`, gives custom types
control over how an error-annotated location will be rendered to an error message.
