use code_gen::rust::{
    gen_getter_fn_copy, gen_setter_fn_copy, Function, ImplBlock, TypeTag as RustType, WithComments,
    WithFunctions,
};
use code_gen::WithName;

use crate::gen::rust::{Naming, Typing};
use crate::gen::Error;
use crate::tree::{Message, MessageField, PrimitiveType, TypeTag, WithTypeTag};

/// Responsible for generating field impl blocks for message fields.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct GenMessageField<'a> {
    naming: &'a Naming,
    typing: &'a Typing,
}

impl<'a> GenMessageField<'a> {
    //! Construction

    /// Creates a new gen message field.
    pub const fn new(naming: &'a Naming, typing: &'a Typing) -> Self {
        Self { naming, typing }
    }
}

impl<'a> GenMessageField<'a> {
    //! Struct

    /// Generates the struct declaration for the message type.
    pub fn gen_field(&self, message: &Message, field: &MessageField) -> Result<ImplBlock, Error> {
        let mut block: ImplBlock = self.naming.type_name(message.name())?.into();

        block.add_comment(format!("Field: {}", field));

        let field_type: &TypeTag = field.type_tag();
        match field_type {
            TypeTag::Primitive(primitive) => match primitive {
                PrimitiveType::UnsignedInt8 => self.gen_copy(field, &mut block)?,
                PrimitiveType::UnsignedInt16 => self.gen_copy(field, &mut block)?,
                PrimitiveType::UnsignedInt32 => self.gen_copy(field, &mut block)?,
                PrimitiveType::UnsignedInt64 => self.gen_copy(field, &mut block)?,
            },
        }

        Ok(block)
    }
}

impl<'a> GenMessageField<'a> {
    //! UnsignedInt8

    /// Generates the impl block for a field that is `Copy`.
    fn gen_copy(&self, field: &MessageField, block: &mut ImplBlock) -> Result<(), Error> {
        let field_name: String = self.naming.field_name(field.name())?;
        let rust_type: RustType = self.typing.field_type(field.type_tag())?;

        let getter: Function = gen_getter_fn_copy(field_name.clone(), rust_type.clone())
            .with_comment(format!("Gets the '{}' field.", field.name()));
        block.add_function(getter);

        let setter: Function = gen_setter_fn_copy(field_name, rust_type)
            .with_comment(format!("Sets the '{}' field.", field.name()));
        block.add_function(setter);

        Ok(())
    }
}
