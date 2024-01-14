use crate::CodeBuffer;

/// A type of comment.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CommentType {
    Line,
    InnerLineDoc,
    OuterLineDoc,
}

impl Default for CommentType {
    fn default() -> Self {
        Self::Line
    }
}

impl CommentType {
    //! Write Line

    /// Writes the line.
    pub fn write_line(&self, b: &mut CodeBuffer, level: usize, line: &str) {
        b.indent(level);
        match self {
            Self::Line => b.write("// "),
            Self::InnerLineDoc => b.write("//! "),
            Self::OuterLineDoc => b.write("/// "),
        }
        b.write(line);
        b.end_line();
    }
}
