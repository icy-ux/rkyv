use crate::{ArchivePointee, Fallible};
use bytecheck::CheckBytes;
use core::fmt;
#[cfg(feature = "std")]
use std::error::Error;

/// Errors that can occur while chechking archived owned pointers
#[derive(Debug)]
pub enum OwnedPointerError<T, R, C> {
    /// The pointer failed to validate due to invalid metadata.
    PointerCheckBytesError(T),
    /// The value pointed to by the owned pointer was invalid.
    ValueCheckBytesError(R),
    /// An error occurred from the validation context.
    ContextError(C),
}

impl<T: fmt::Display, R: fmt::Display, C: fmt::Display> fmt::Display
    for OwnedPointerError<T, R, C>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OwnedPointerError::PointerCheckBytesError(e) => e.fmt(f),
            OwnedPointerError::ValueCheckBytesError(e) => e.fmt(f),
            OwnedPointerError::ContextError(e) => e.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl<T: Error + 'static, R: Error + 'static, C: Error + 'static> Error
    for OwnedPointerError<T, R, C>
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            OwnedPointerError::PointerCheckBytesError(e) => Some(e as &dyn Error),
            OwnedPointerError::ValueCheckBytesError(e) => Some(e as &dyn Error),
            OwnedPointerError::ContextError(e) => Some(e as &dyn Error),
        }
    }
}

pub type CheckOwnedPointerError<T, C> = OwnedPointerError<
    <<T as ArchivePointee>::ArchivedMetadata as CheckBytes<C>>::Error,
    <T as CheckBytes<C>>::Error,
    <C as Fallible>::Error,
>;