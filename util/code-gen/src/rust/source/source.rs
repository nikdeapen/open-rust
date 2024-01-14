use crate::{CodeBuffer, Statement, WithStatements};

/// A source file.
#[derive(Default)]
pub struct Source {
    statements: Vec<Box<dyn Statement>>,
}

impl WithStatements for Source {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for Source {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        if let Some((first, rest)) = self.statements.split_first() {
            first.write(b, level);
            for statement in rest {
                b.end_line();
                statement.write(b, level);
            }
        }
    }
}
