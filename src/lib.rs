mod err_at;
mod error;
mod location;

/// Location-annotated error
pub use error::Error;

/// Extension-trait providing `err_at` convenience method for location-annotating `Result`s
pub use err_at::ErrAt;

/// Location that an `Error` can occur
pub use location::Location;

/// Location-annotated Result
pub type Result<T, E, L> = std::result::Result<T, Error<E, L>>;
