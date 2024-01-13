use crate::rust::PrimitiveType;
use crate::{CodeBuffer, Expression, ToStaticStr};

/// A type tag.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeTag {
    /// A primitive type.
    Primitive(PrimitiveType),

    /// A named type.
    Named(String),

    /// A generic type.
    Generic((Box<TypeTag>, Vec<TypeTag>)),
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

impl TypeTag {
    //! Conversions

    /// Converts the type to an `Option` of itself.
    pub fn to_option(self) -> Self {
        Self::Generic((Box::new("Option".into()), vec![self]))
    }

    /// Converts the type to an `Vec` of itself.
    pub fn to_vec(self) -> Self {
        Self::Generic((Box::new("Vec".into()), vec![self]))
    }
}

impl TypeTag {
    //! Mutations

    /// Adds the generic type.
    pub fn with_generic<T>(self, generic: T) -> Self
    where
        T: Into<TypeTag>,
    {
        match self {
            Self::Generic((base, mut generics)) => {
                generics.push(generic.into());
                Self::Generic((base, generics))
            }
            _ => Self::Generic((Box::new(self), Vec::with_capacity(2))).with_generic(generic),
        }
    }
}

impl Expression for TypeTag {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Self::Primitive(primitive) => b.write(primitive.to_static_str()),
            Self::Named(name) => b.write(name.as_str()),
            Self::Generic((base, generics)) => {
                base.write(b);
                if let Some((first, rest)) = generics.split_first() {
                    b.write("<");
                    first.write(b);
                    for generic in rest {
                        b.write(", ");
                        generic.write(b);
                    }
                    b.write(">");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::PrimitiveType::*;
    use crate::rust::TypeTag;
    use crate::rust::TypeTag::Generic;
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

    #[test]
    fn write_generic() {
        let tag: TypeTag = Generic((Box::new("MyType".into()), Vec::default()));
        let result: String = CodeBuffer::display_expression(&tag);
        assert_eq!(result, "MyType");

        let tag: TypeTag = tag.with_generic(UnsignedInt8);
        let result: String = CodeBuffer::display_expression(&tag);
        assert_eq!(result, "MyType<u8>");

        let tag: TypeTag = tag.with_generic(UnsignedInt16);
        let result: String = CodeBuffer::display_expression(&tag);
        assert_eq!(result, "MyType<u8, u16>");

        let tag: TypeTag = tag.with_generic(UnsignedInt32);
        let result: String = CodeBuffer::display_expression(&tag);
        assert_eq!(result, "MyType<u8, u16, u32>");
    }
}
