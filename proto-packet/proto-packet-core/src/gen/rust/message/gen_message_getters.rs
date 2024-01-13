use code_gen::rust::{gen_getter_fn_copy, Function, ImplBlock, TypeTag as RustType, WithFunctions};
use code_gen::WithName;

use crate::gen::rust::{Naming, Typing};
use crate::gen::Error;
use crate::tree::{Message, MessageField, WithMessageFields, WithTypeTag};

/// Responsible for generating the getters impl block message types.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct GenMessageGetters<'a> {
    naming: &'a Naming,
    typing: &'a Typing,
}

impl<'a> GenMessageGetters<'a> {
    //! Construction

    /// Creates a new gen message getters.
    pub const fn new(naming: &'a Naming, typing: &'a Typing) -> Self {
        Self { naming, typing }
    }
}

impl<'a> GenMessageGetters<'a> {
    //! Getters

    /// Generates the getters impl block for the message.
    pub fn gen_getters(&self, message: &Message) -> Result<ImplBlock, Error> {
        let mut block: ImplBlock = self.naming.type_name(message.name())?.into();

        for field in message.fields() {
            block.add_function(self.gen_getter_fn(field)?);
        }

        Ok(block)
    }
}

impl<'a> GenMessageGetters<'a> {
    //! Getter Functions

    /// Generates the getters impl block for the message.
    fn gen_getter_fn(&self, field: &MessageField) -> Result<Function, Error> {
        if self.typing.is_copy(field.type_tag()) {
            let field_name: String = self.naming.field_name(field.name())?;
            let field_type: RustType = self.typing.field_type(field.type_tag())?;
            Ok(gen_getter_fn_copy(field_name, field_type))
        } else {
            Err(Error::UnrecognizedType(field.type_tag().clone()))
        }
    }
}
