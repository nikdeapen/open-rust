use code_gen::rust::Source;
use code_gen::WithStatements;

use crate::gen::rust::message::gen_message_field::GenMessageField;
use crate::gen::rust::{GenMessageStruct, Naming, Typing};
use crate::gen::Error;
use crate::tree::{Message, WithMessageFields};

/// Responsible for generating code for message types.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct GenMessage {
    naming: Naming,
    typing: Typing,
}

impl GenMessage {
    //! Construction

    /// Creates a new gen message struct.
    pub const fn new(naming: Naming, typing: Typing) -> Self {
        Self { naming, typing }
    }
}

impl GenMessage {
    //! Source

    /// Generates the source.
    pub fn gen_source(&self, message: &Message) -> Result<Source, Error> {
        let mut source: Source = Source::default();

        let gen_struct: GenMessageStruct = GenMessageStruct::new(&self.naming, &self.typing);
        source.add_statement(gen_struct.gen_struct(message)?);

        let gen_field: GenMessageField = GenMessageField::new(&self.naming, &self.typing);
        for field in message.fields() {
            source.add_statement(gen_field.gen_field(message, field)?);
        }

        Ok(source)
    }
}
