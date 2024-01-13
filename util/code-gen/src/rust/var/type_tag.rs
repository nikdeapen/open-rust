use crate::rust::PrimitiveType;
use crate::{CodeBuffer, Expression, ToStaticStr};

/// A type tag.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeTag {
    /// A primitive type.
    Primitive(PrimitiveType),

    /// A named type.
    Named(String),
}

impl From<PrimitiveType> for TypeTag {
    fn from(primitive: PrimitiveType) -> Self {
        Self::Primitive(primitive)
    }
}

impl<S: Into<String>> From<S> for TypeTag {
    fn from(name: S) -> Self {
        Self::Named(name.into())
    }
}

impl Expression for TypeTag {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Self::Primitive(primitive) => b.write(primitive.to_static_str()),
            Self::Named(name) => b.write(name.as_str()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::PrimitiveType::UnsignedInt8;
    use crate::rust::TypeTag;
    use crate::CodeBuffer;

    #[test]
    fn write() {
        let tag: TypeTag = UnsignedInt8.into();
        let result: String = CodeBuffer::display_expression(&tag);
        assert_eq!(result, "u8");

        let tag: TypeTag = "MyType".into();
        let result: String = CodeBuffer::display_expression(&tag);
        assert_eq!(result, "MyType");
    }
}
