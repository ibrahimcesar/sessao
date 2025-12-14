//! Lexer for tokenizing `.sessao` source files.

use crate::{Error, Result, Span};

/// A token produced by the lexer.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The kind of token
    pub kind: TokenKind,
    /// The source span
    pub span: Span,
    /// The token text (for identifiers and literals)
    pub text: String,
}

/// The kind of token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // Keywords
    /// `protocol`
    Protocol,
    /// `roles`
    Roles,
    /// `phase`
    Phase,
    /// `choice`
    Choice,
    /// `match`
    Match,
    /// `continue`
    Continue,
    /// `end`
    End,
    /// `when`
    When,
    /// `parallel`
    Parallel,
    /// `reliable`
    Reliable,
    /// `unreliable`
    Unreliable,
    /// `type`
    Type,
    /// `true`
    True,
    /// `false`
    False,

    // Symbols
    /// `->`
    Arrow,
    /// `=>`
    FatArrow,
    /// `:`
    Colon,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `@`
    At,
    /// `?`
    Question,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `(`
    LParen,
    /// `)`
    RParen,

    // Literals and identifiers
    /// Identifier
    Ident,
    /// String literal
    String,
    /// Integer literal
    Integer,
    /// Float literal
    Float,

    // Special
    /// End of file
    Eof,
    /// Unknown/error token
    Error,
}

/// Lexer for `.sessao` files.
pub struct Lexer<'a> {
    source: &'a str,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given source.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenize the entire source into a vector of tokens.
    pub fn tokenize(source: &str) -> Result<Vec<Token>> {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();

        loop {
            let token = lexer.next_token()?;
            let is_eof = token.kind == TokenKind::Eof;
            tokens.push(token);
            if is_eof {
                break;
            }
        }

        Ok(tokens)
    }

    /// Get the next token from the source.
    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace_and_comments();

        if self.is_at_end() {
            return Ok(Token {
                kind: TokenKind::Eof,
                span: self.current_span(),
                text: String::new(),
            });
        }

        // TODO: Implement tokenization
        Err(Error::NotImplemented("Lexer not yet implemented".into()))
    }

    fn skip_whitespace_and_comments(&mut self) {
        // TODO: Implement
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }

    fn current_span(&self) -> Span {
        Span::new(self.position, self.position, self.line, self.column)
    }
}
