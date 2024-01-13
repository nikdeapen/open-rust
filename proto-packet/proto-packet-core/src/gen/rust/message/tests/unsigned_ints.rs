use code_gen::rust::{ImplBlock, Struct};
use code_gen::{CodeBuffer, Statement};

use crate::gen::rust::{GenMessageGetters, GenMessageStruct, Naming, Typing};
use crate::gen::Error;
use crate::tree::PrimitiveType::{UnsignedInt16, UnsignedInt32, UnsignedInt64, UnsignedInt8};
use crate::tree::{Message, MessageField, WithFieldNumber, WithMessageFields};

#[test]
fn test() -> Result<(), Error> {
    let mut message: Message = "UnsignedInts".into();

    message.add_field(MessageField::from(("one", UnsignedInt8)).with_field_number(1));
    message.add_field(MessageField::from(("two", UnsignedInt16)).with_field_number(2));
    message.add_field(MessageField::from(("three", UnsignedInt32)).with_field_number(3));
    message.add_field(MessageField::from(("four", UnsignedInt64)).with_field_number(4));

    let naming: Naming = Naming::default();
    let typing: Typing = Typing::default();
    let structure: Struct = GenMessageStruct::new(&naming, &typing).gen_struct(&message)?;
    let getters: ImplBlock = GenMessageGetters::new(&naming, &typing).gen_getters(&message)?;

    let mut b: CodeBuffer = CodeBuffer::new("    ", "\n", 1024);
    structure.write(&mut b, 0);
    b.end_line();
    getters.write(&mut b, 0);

    let result: String = b.export();
    let expected: &str = include_str!("unsigned_ints.txt");
    assert_eq!(result, expected);

    Ok(())
}
