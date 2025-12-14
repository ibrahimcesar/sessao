//! # sessao-lang
//!
//! Protocol Definition Language parser and AST for Sessão.
//!
//! This crate provides:
//! - Lexer for tokenizing `.sessao` files
//! - Parser for building the Abstract Syntax Tree (AST)
//! - AST types representing all language constructs
//! - Semantic analysis and type checking
//! - Error diagnostics with source locations
//!
//! ## Example
//!
//! ```ignore
//! use sessao_lang::{parse, SourceFile};
//!
//! let source = r#"
//!     protocol Hello {
//!         roles Client, Server
//!         phase Main {
//!             Client -> Server: Ping
//!             Server -> Client: Pong
//!             end
//!         }
//!     }
//! "#;
//!
//! let protocol = parse(source)?;
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod span;

mod error;

pub use error::{Error, Result};
pub use span::Span;

/// Parse a `.sessao` source string into an AST.
///
/// # Errors
///
/// Returns an error if the source contains syntax errors.
pub fn parse(_source: &str) -> Result<ast::Protocol> {
    todo!("Implement parser")
}

/// Parse and validate a `.sessao` source string.
///
/// This performs both syntactic parsing and semantic analysis.
///
/// # Errors
///
/// Returns an error if the source contains syntax or semantic errors.
pub fn parse_and_validate(_source: &str) -> Result<ast::Protocol> {
    todo!("Implement parser and semantic analysis")
}
