use std::fmt::{Display, Formatter};

use code_gen::WithName;

use crate::tree::{MessageField, WithMessageFields};

/// A message type.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Message {
    name: String,
    fields: Vec<MessageField>,
}

impl<S: Into<String>> From<S> for Message {
    fn from(name: S) -> Self {
        Self {
            name: name.into(),
            fields: Vec::default(),
        }
    }
}

impl WithName for Message {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithMessageFields for Message {
    fn fields(&self) -> &[MessageField] {
        self.fields.as_slice()
    }

    fn add_field<F>(&mut self, field: F)
    where
        F: Into<MessageField>,
    {
        self.fields.push(field.into());
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "type {} message {{", self.name)?;
        if self.fields.is_empty() {
            write!(f, "}}\n")
        } else {
            write!(f, "\n")?;
            for field in self.fields() {
                write!(f, "\t{}\n", field)?;
            }
            write!(f, "}}\n")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::PrimitiveType::{UnsignedInt16, UnsignedInt8};
    use crate::tree::{Message, WithMessageFields};

    #[test]
    fn display() {
        let message: Message = "MyMessage".into();
        let result: String = message.to_string();
        let expected: &str = "type MyMessage message {}\n";
        assert_eq!(result, expected);

        let message: Message = message.with_field(("one", UnsignedInt8));
        let result: String = message.to_string();
        let expected: &str = "type MyMessage message {\n\tone: u8;\n}\n";
        assert_eq!(result, expected);

        let message: Message = message.with_field(("two", UnsignedInt16));
        let result: String = message.to_string();
        let expected: &str = "type MyMessage message {\n\tone: u8;\n\ttwo: u16;\n}\n";
        assert_eq!(result, expected);
    }
}
