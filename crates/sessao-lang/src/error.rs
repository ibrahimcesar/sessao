//! Error types for the Sessão language.

use crate::Span;
use miette::Diagnostic;
use thiserror::Error;

/// Result type for Sessão operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during parsing and analysis.
#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    /// A syntax error in the source.
    #[error("syntax error: {message}")]
    #[diagnostic(code(sessao::syntax))]
    Syntax {
        /// Error message
        message: String,
        /// Source location
        #[label("here")]
        span: Span,
    },

    /// An unexpected token was encountered.
    #[error("unexpected token: expected {expected}, found {found}")]
    #[diagnostic(code(sessao::unexpected_token))]
    UnexpectedToken {
        /// What was expected
        expected: String,
        /// What was found
        found: String,
        /// Source location
        #[label("unexpected token")]
        span: Span,
    },

    /// An undefined reference was found.
    #[error("undefined {kind}: '{name}'")]
    #[diagnostic(code(sessao::undefined))]
    Undefined {
        /// Kind of thing that's undefined (role, phase, type)
        kind: String,
        /// Name that wasn't found
        name: String,
        /// Source location
        #[label("not defined")]
        span: Span,
    },

    /// A duplicate definition was found.
    #[error("duplicate {kind}: '{name}'")]
    #[diagnostic(code(sessao::duplicate))]
    Duplicate {
        /// Kind of thing that's duplicated
        kind: String,
        /// Name that's duplicated
        name: String,
        /// Source location of the duplicate
        #[label("duplicate definition")]
        span: Span,
    },

    /// A type error.
    #[error("type error: {message}")]
    #[diagnostic(code(sessao::type_error))]
    Type {
        /// Error message
        message: String,
        /// Source location
        #[label("type error")]
        span: Span,
    },

    /// Feature not yet implemented.
    #[error("not implemented: {0}")]
    #[diagnostic(code(sessao::not_implemented))]
    NotImplemented(String),

    /// Internal compiler error.
    #[error("internal error: {0}")]
    #[diagnostic(code(sessao::internal))]
    Internal(String),
}

impl Error {
    /// Create a syntax error.
    pub fn syntax(message: impl Into<String>, span: Span) -> Self {
        Self::Syntax {
            message: message.into(),
            span,
        }
    }

    /// Create an unexpected token error.
    pub fn unexpected(expected: impl Into<String>, found: impl Into<String>, span: Span) -> Self {
        Self::UnexpectedToken {
            expected: expected.into(),
            found: found.into(),
            span,
        }
    }

    /// Create an undefined reference error.
    pub fn undefined(kind: impl Into<String>, name: impl Into<String>, span: Span) -> Self {
        Self::Undefined {
            kind: kind.into(),
            name: name.into(),
            span,
        }
    }

    /// Create a duplicate definition error.
    pub fn duplicate(kind: impl Into<String>, name: impl Into<String>, span: Span) -> Self {
        Self::Duplicate {
            kind: kind.into(),
            name: name.into(),
            span,
        }
    }

    /// Create a type error.
    pub fn type_error(message: impl Into<String>, span: Span) -> Self {
        Self::Type {
            message: message.into(),
            span,
        }
    }
}
