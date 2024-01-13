/// An element with an optional field number.
pub trait WithFieldNumber: Sized {
    /// Gets the optional field number.
    fn field_number(&self) -> Option<u32>;

    /// Sets the field number.
    fn with_field_number(mut self, field_number: u32) -> Self {
        self.set_field_number(field_number);
        self
    }

    /// Sets the field number.
    fn set_field_number(&mut self, field_number: u32);
}
