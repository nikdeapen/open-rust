use code_gen::rust::Access::Public;
use code_gen::rust::{
    Struct, StructField, TypeTag as RustType, WithAccess, WithDerives, WithStructFields,
};
use code_gen::WithName;

use crate::gen::rust::{Naming, Typing};
use crate::gen::Error;
use crate::tree::{Message, MessageField, WithMessageFields, WithTypeTag};

/// Responsible for generating struct declarations for message types.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct GenMessageStruct<'a> {
    naming: &'a Naming,
    typing: &'a Typing,
}

impl<'a> GenMessageStruct<'a> {
    //! Construction

    /// Creates a new gen message struct.
    pub const fn new(naming: &'a Naming, typing: &'a Typing) -> Self {
        Self { naming, typing }
    }
}

impl<'a> GenMessageStruct<'a> {
    //! Struct

    /// Generates the struct declaration for the message type.
    pub fn gen_struct(&self, message: &Message) -> Result<Struct, Error> {
        let mut structure: Struct =
            Struct::from(self.naming.type_name(message.name())?).with_access(Public);

        self.gen_derives(message, &mut structure)?;

        for field in message.fields() {
            structure.add_field(self.gen_field(field)?);
        }

        Ok(structure)
    }
}

impl<'a> GenMessageStruct<'a> {
    //! Derives

    fn gen_derives(&self, message: &Message, structure: &mut Struct) -> Result<(), Error> {
        let copy: bool = message
            .fields()
            .iter()
            .all(|f| self.typing.is_copy(f.type_tag()));
        if copy {
            structure.add_derive("Copy");
        }
        structure.add_derive("Clone");
        structure.add_derive("Ord");
        structure.add_derive("PartialOrd");
        structure.add_derive("Eq");
        structure.add_derive("PartialEq");
        structure.add_derive("Hash");
        structure.add_derive("Debug");

        Ok(())
    }
}

impl<'a> GenMessageStruct<'a> {
    //! Fields

    /// Generates the struct field for the message field.
    fn gen_field(&self, field: &MessageField) -> Result<StructField, Error> {
        let name: String = self.naming.type_name(field.name())?;
        let type_tag: RustType = self.typing.field_type(field.type_tag())?;
        Ok(StructField::from((name, type_tag)))
    }
}
