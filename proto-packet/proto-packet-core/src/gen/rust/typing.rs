use code_gen::rust::{PrimitiveType as RustPrimitive, TypeTag as RustType};

use crate::gen::Error;
use crate::tree::{PrimitiveType, TypeTag};

/// Responsible for resolving types.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct Typing {
    _nothing: (),
}

impl Typing {
    //! Field Types

    /// Gets the rust field type for the declared field type.
    pub fn field_type(&self, declared_type: &TypeTag) -> Result<RustType, Error> {
        Ok(match declared_type {
            TypeTag::Primitive(primitive) => match primitive {
                PrimitiveType::UnsignedInt8 => RustPrimitive::UnsignedInt8.to_type_tag(),
                PrimitiveType::UnsignedInt16 => RustPrimitive::UnsignedInt16.to_type_tag(),
                PrimitiveType::UnsignedInt32 => RustPrimitive::UnsignedInt32.to_type_tag(),
                PrimitiveType::UnsignedInt64 => RustPrimitive::UnsignedInt64.to_type_tag(),
            },
        })
    }
}
