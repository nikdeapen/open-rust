use crate::rust::CommentType;
use crate::CodeBuffer;

/// An element with comments.
pub trait WithComments: Sized {
    /// Gets the comment lines.
    fn comment_lines(&self) -> &[String];

    /// Adds the comment line.
    fn with_comment<S>(mut self, comment_line: S) -> Self
    where
        S: Into<String>,
    {
        self.add_comment(comment_line);
        self
    }

    /// Adds the comment line.
    fn add_comment<S>(&mut self, comment_line: S)
    where
        S: Into<String>;

    /// Writes the comment.
    fn write_comments(&self, comment_type: CommentType, b: &mut CodeBuffer, level: usize) {
        for line in self.comment_lines() {
            comment_type.write_line(b, level, line.as_str());
        }
    }
}
