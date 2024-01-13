use crate::tree::TypeTag;

/// An error generating code.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Error {
    /// An unrecognized type could not be processed.
    UnrecognizedType(TypeTag),
}
