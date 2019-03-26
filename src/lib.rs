//! `oi` provides location-annotated `io::Errors` so you can display useful error messages to your users.
//!
//! Error messages without location information are often not actionable, especially when they come from a complex program with many potential sources of errors.
//!
//! Compare an unannotated error:
//!
//! ```sh
//! $ foo
//! No such file or directory (os error 2)
//! ```
//!
//! …with an annotated error:
//!
//! ```sh
//! $ foo
//! Configuration.toml: No such file or directory (os error 2)
//! ```
//!
//! Aside from being "io" backwards, `oi` is also an exclamation, as in [Oi! Oi! Oi!](https://youtu.be/XWLU76o5rEI). Imagine alerting your users to the location of errors: "Oi! 1.2.3.4 is unreachable!"

mod err_at;
mod error;
mod location;

/// Location-annotated `io::Error`
pub use error::Error;

/// Location that an io::Error can occur
pub use location::Location;

/// Extension-trait providing `err_at` convenience method for location-annotating `io::Result`s
pub use err_at::ErrAt;

/// Location-annotated Result
pub type Result<T> = std::result::Result<T, Error>;
