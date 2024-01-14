/// An element that can be empty.
pub trait IsEmpty {
    /// Checks if the element is empty.
    fn is_empty(&self) -> bool;
}
