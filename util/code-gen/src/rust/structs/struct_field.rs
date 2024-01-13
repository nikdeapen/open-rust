use crate::rust::{TypeTag, Var, WithTypeTag, WithVar};
use crate::{CodeBuffer, Statement, WithName};

/// A field of a struct.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct StructField {
    var: Var,
}

impl<V: Into<Var>> From<V> for StructField {
    fn from(var: V) -> Self {
        Self { var: var.into() }
    }
}

impl WithName for StructField {
    fn name(&self) -> &str {
        self.var.name()
    }
}

impl WithTypeTag for StructField {
    fn type_tag(&self) -> &TypeTag {
        self.var.type_tag()
    }
}

impl WithVar for StructField {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl Statement for StructField {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        self.write_var(b);
        b.write(",");
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::PrimitiveType::UnsignedInt32;
    use crate::rust::StructField;
    use crate::CodeBuffer;

    #[test]
    fn write() {
        let field: StructField = ("one", UnsignedInt32).into();
        let result: String = CodeBuffer::display_statement(&field);
        assert_eq!(result, "one: u32,\n");
    }
}
