use crate::{error::Error, location::Location};

use failure::Fail;

pub trait ErrAt<T, E: Fail> {
  /// Iff self.is_err() annotate with location
  fn err_at<L: Location, I: Into<L>>(self, location: I) -> Result<T, Error<E, L>>;
}

impl<T, E: Fail> ErrAt<T, E> for Result<T, E> {
  fn err_at<L: Location, I: Into<L>>(self, location: I) -> Result<T, Error<E, L>> {
    match self {
      Ok(ok) => Ok(ok),
      Err(error) => Err(Error {
        error,
        location: location.into(),
      }),
    }
  }
}
