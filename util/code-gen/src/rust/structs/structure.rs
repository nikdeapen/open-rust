use crate::rust::structs::with_derives::WithDerives;
use crate::rust::{Access, StructField, WithAccess, WithStructFields};
use crate::{CodeBuffer, Statement, WithName};

/// A struct declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Struct {
    derives: Vec<String>,
    access: Access,
    name: String,
    fields: Vec<StructField>,
}

impl<S: Into<String>> From<S> for Struct {
    fn from(name: S) -> Self {
        Self {
            derives: Vec::default(),
            access: Access::default(),
            name: name.into(),
            fields: Vec::default(),
        }
    }
}

impl WithDerives for Struct {
    fn derives(&self) -> &[String] {
        self.derives.as_slice()
    }

    fn add_derive<S>(&mut self, derive: S)
    where
        S: Into<String>,
    {
        self.derives.push(derive.into());
    }
}

impl WithAccess for Struct {
    fn access(&self) -> &Access {
        &self.access
    }

    fn set_access<A>(&mut self, access: A)
    where
        A: Into<Access>,
    {
        self.access = access.into();
    }
}

impl WithName for Struct {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithStructFields for Struct {
    fn fields(&self) -> &[StructField] {
        self.fields.as_slice()
    }

    fn add_field<F>(&mut self, field: F)
    where
        F: Into<StructField>,
    {
        self.fields.push(field.into());
    }
}

impl Statement for Struct {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_derives(b, level);
        b.indent(level);
        self.write_access(b);
        b.write("struct ");
        self.write_name(b);
        b.write(" {");
        if self.fields.is_empty() {
            b.write("}");
            b.end_line();
        } else {
            b.end_line();
            self.write_fields(b, level + 1);
            b.line(level, "}");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::structs::with_derives::WithDerives;
    use crate::rust::PrimitiveType::*;
    use crate::rust::{Access, Struct, WithAccess, WithStructFields};
    use crate::CodeBuffer;

    #[test]
    fn write_empty() {
        let structure: Struct = "MyStruct".into();
        let result: String = CodeBuffer::display_statement(&structure);
        assert_eq!(result, "struct MyStruct {}\n");
    }

    #[test]
    fn write_derives() {
        let structure: Struct = Struct::from("MyStruct").with_derive("Copy");
        let result: String = CodeBuffer::display_statement(&structure);
        assert_eq!(result, "#[derive(Copy)]\nstruct MyStruct {}\n");

        let structure: Struct = structure.with_derive("Clone");
        let result: String = CodeBuffer::display_statement(&structure);
        assert_eq!(result, "#[derive(Copy, Clone)]\nstruct MyStruct {}\n");

        let structure: Struct = structure.with_derive("Debug");
        let result: String = CodeBuffer::display_statement(&structure);
        assert_eq!(
            result,
            "#[derive(Copy, Clone, Debug)]\nstruct MyStruct {}\n"
        );
    }

    #[test]
    fn write_access() {
        let structure: Struct = Struct::from("MyStruct").with_access(Access::Public);
        let result: String = CodeBuffer::display_statement(&structure);
        assert_eq!(result, "pub struct MyStruct {}\n");
    }

    #[test]
    fn write_fields() {
        let structure: Struct = Struct::from("MyStruct")
            .with_field(("one", UnsignedInt8))
            .with_field(("two", UnsignedInt16));
        let result: String = CodeBuffer::display_statement(&structure);
        assert_eq!(result, "struct MyStruct {\n\tone: u8,\n\ttwo: u16,\n}\n");
    }
}
