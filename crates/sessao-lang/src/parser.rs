//! Parser for building the AST from tokens.

use crate::ast::Protocol;
use crate::lexer::Token;
use crate::{Error, Result};

/// Parser for `.sessao` files.
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Create a new parser from tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Parse the tokens into a protocol AST.
    pub fn parse(&mut self) -> Result<Protocol> {
        // TODO: Implement parsing
        Err(Error::NotImplemented("Parser not yet implemented".into()))
    }
}
