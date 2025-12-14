//! Source location tracking.

use miette::SourceSpan;

/// A span representing a range in the source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    /// Start byte offset
    pub start: usize,
    /// End byte offset (exclusive)
    pub end: usize,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
}

impl From<Span> for SourceSpan {
    fn from(span: Span) -> Self {
        SourceSpan::from((span.start, span.end.saturating_sub(span.start)))
    }
}

impl Span {
    /// Create a new span.
    pub const fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }

    /// Create a dummy span for testing.
    pub const fn dummy() -> Self {
        Self {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        }
    }

    /// Merge two spans into one that covers both.
    pub fn merge(self, other: Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            line: self.line.min(other.line),
            column: if self.line <= other.line {
                self.column
            } else {
                other.column
            },
        }
    }

    /// Get the length of the span in bytes.
    pub const fn len(&self) -> usize {
        self.end - self.start
    }

    /// Check if the span is empty.
    pub const fn is_empty(&self) -> bool {
        self.start == self.end
    }
}
