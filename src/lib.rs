//! `oi` provides a location-annotated error type so you can display useful error messages to your users.
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
//! `oi` is named after the exclamation, as in [Oi! Oi! Oi!](https://youtu.be/XWLU76o5rEI). Imagine alerting your users to the location of errors: "Oi! 1.2.3.4 is unreachable!"

mod err_at;
mod error;
mod location;
mod site;

/// Location-annotated error
pub use error::Error;

/// Extension-trait providing `err_at` convenience method for location-annotating `Result`s
pub use err_at::ErrAt;

/// Location that an `Error` can occur
pub use location::Location;

/// Optional enum for use as Location with variants for common io error locations
pub use site::Site;

/// Location-annotated Result
pub type Result<T, E, L> = std::result::Result<T, Error<E, L>>;
