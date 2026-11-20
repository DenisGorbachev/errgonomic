use crate::writeln_error_to_formatter;
use core::fmt::{self, Display, Formatter};
use std::error::Error;

pub struct ErrorDisplayer<'a, E: ?Sized>(pub &'a E);

impl<E: Error + ?Sized> Display for ErrorDisplayer<'_, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln_error_to_formatter(self.0, f)
    }
}

impl<'a, E: Error + ?Sized> From<&'a E> for ErrorDisplayer<'a, E> {
    fn from(error: &'a E) -> Self {
        Self(error)
    }
}
