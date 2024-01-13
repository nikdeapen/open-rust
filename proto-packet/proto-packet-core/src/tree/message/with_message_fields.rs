use crate::tree::MessageField;

/// An element with message fields.
pub trait WithMessageFields: Sized {
    /// Gets the message fields.
    fn fields(&self) -> &[MessageField];

    /// Adds the message field.
    fn with_field<F>(mut self, field: F) -> Self
    where
        F: Into<MessageField>,
    {
        self.add_field(field);
        self
    }

    /// Adds the message field.
    fn add_field<F>(&mut self, field: F)
    where
        F: Into<MessageField>;
}
