use crate::rust::{Function, TypeTag, WithFunctions};
use crate::{CodeBuffer, Expression, Statement};

/// A struct impl block.
pub struct ImplBlock {
    base: TypeTag,
    functions: Vec<Function>,
}

impl<T: Into<TypeTag>> From<T> for ImplBlock {
    fn from(base: T) -> Self {
        Self {
            base: base.into(),
            functions: Vec::default(),
        }
    }
}

impl WithFunctions for ImplBlock {
    fn functions(&self) -> &[Function] {
        self.functions.as_slice()
    }

    fn add_function<F>(&mut self, function: F)
    where
        F: Into<Function>,
    {
        self.functions.push(function.into());
    }
}

impl Statement for ImplBlock {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("impl ");
        self.base.write(b);
        b.write(" {");
        if self.functions.is_empty() {
            b.write("}");
            b.end_line();
        } else {
            b.end_line();
            self.write_functions(b, level + 1);
            b.line(level, "}");
        }
    }
}
