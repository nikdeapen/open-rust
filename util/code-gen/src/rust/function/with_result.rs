use crate::rust::TypeTag;
use crate::{CodeBuffer, Expression};

/// An element with an optional result.
pub trait WithResult {
    /// Gets the optional result.
    fn result(&self) -> Option<&TypeTag>;

    /// Sets the result.
    fn with_result<T>(self, result: T) -> Self
    where
        T: Into<TypeTag>;

    /// Sets the result.
    fn set_result<T>(&mut self, result: T)
    where
        T: Into<TypeTag>;

    /// Writes the optional result. (includes the ` -> ` if the result is not None)
    fn write_result(&self, b: &mut CodeBuffer) {
        if let Some(result) = self.result() {
            b.write(" -> ");
            result.write(b);
        }
    }
}
