use crate::rust::{Access, Signature, WithAccess, WithSignature};
use crate::{CodeBuffer, Statement, WithStatements};

/// A function declaration.
pub struct Function {
    access: Access,
    signature: Signature,
    statements: Vec<Box<dyn Statement>>,
}

impl<S: Into<Signature>> From<S> for Function {
    fn from(signature: S) -> Self {
        Self {
            access: Access::default(),
            signature: signature.into(),
            statements: Vec::default(),
        }
    }
}

impl WithAccess for Function {
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

impl WithSignature for Function {
    fn signature(&self) -> &Signature {
        &self.signature
    }
}

impl WithStatements for Function {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for Function {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        self.write_access(b);
        b.write("fn ");
        self.write_signature(b);
        b.space();
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::Access::Public;
    use crate::rust::{Function, WithAccess};
    use crate::{CodeBuffer, Literal, WithStatements};

    #[test]
    fn write_empty() {
        let function: Function = "myFn".into();
        let result: String = CodeBuffer::display_statement(&function);
        assert_eq!(result, "fn myFn() {}\n");
    }

    #[test]
    fn write_access() {
        let function: Function = Function::from("myFn").with_access(Public);
        let result: String = CodeBuffer::display_statement(&function);
        assert_eq!(result, "pub fn myFn() {}\n");
    }

    #[test]
    fn write_statements() {
        let function: Function =
            Function::from("myFn").with_expression_statement(Literal::from("one"));
        let result: String = CodeBuffer::display_statement(&function);
        assert_eq!(result, "fn myFn() {\n\tone\n}\n");

        let function: Function = function.with_expression_statement(Literal::from("two"));
        let result: String = CodeBuffer::display_statement(&function);
        assert_eq!(result, "fn myFn() {\n\tone\n\ttwo\n}\n");
    }
}
