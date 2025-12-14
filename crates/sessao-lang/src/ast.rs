//! Abstract Syntax Tree types for the Sessão PDL.

use crate::Span;

/// A complete protocol definition.
#[derive(Debug, Clone)]
pub struct Protocol {
    /// Protocol name
    pub name: String,
    /// Source span
    pub span: Span,
    /// Declared roles
    pub roles: Vec<Role>,
    /// Type definitions
    pub types: Vec<TypeDef>,
    /// Protocol phases
    pub phases: Vec<Phase>,
}

/// A role in the protocol (e.g., Client, Server).
#[derive(Debug, Clone)]
pub struct Role {
    /// Role name
    pub name: String,
    /// Source span
    pub span: Span,
}

/// A custom type definition.
#[derive(Debug, Clone)]
pub struct TypeDef {
    /// Type name
    pub name: String,
    /// Source span
    pub span: Span,
    /// Type body
    pub body: TypeBody,
}

/// The body of a type definition.
#[derive(Debug, Clone)]
pub enum TypeBody {
    /// Struct with named fields
    Struct(Vec<Field>),
    /// Enum with variants
    Enum(Vec<EnumVariant>),
    /// Type alias
    Alias(Type),
}

/// A field in a struct or message.
#[derive(Debug, Clone)]
pub struct Field {
    /// Field name
    pub name: String,
    /// Source span
    pub span: Span,
    /// Field type
    pub ty: Type,
    /// Whether the field is optional
    pub optional: bool,
}

/// An enum variant.
#[derive(Debug, Clone)]
pub struct EnumVariant {
    /// Variant name
    pub name: String,
    /// Source span
    pub span: Span,
    /// Associated data (if any)
    pub fields: Vec<Field>,
}

/// A type expression.
#[derive(Debug, Clone)]
pub enum Type {
    /// Primitive type (bool, u32, string, etc.)
    Primitive(PrimitiveType),
    /// Array type [T]
    Array(Box<Type>),
    /// Map type {K: V}
    Map(Box<Type>, Box<Type>),
    /// Optional type T?
    Optional(Box<Type>),
    /// Reference to a custom type
    Named(String),
}

/// Built-in primitive types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    /// Boolean
    Bool,
    /// Unsigned 8-bit integer
    U8,
    /// Unsigned 16-bit integer
    U16,
    /// Unsigned 32-bit integer
    U32,
    /// Unsigned 64-bit integer
    U64,
    /// Signed 8-bit integer
    I8,
    /// Signed 16-bit integer
    I16,
    /// Signed 32-bit integer
    I32,
    /// Signed 64-bit integer
    I64,
    /// 32-bit floating point
    F32,
    /// 64-bit floating point
    F64,
    /// UTF-8 string
    String,
    /// UUID
    Uuid,
    /// Timestamp
    Timestamp,
    /// Raw bytes
    Bytes,
}

/// A protocol phase.
#[derive(Debug, Clone)]
pub struct Phase {
    /// Phase name
    pub name: String,
    /// Source span
    pub span: Span,
    /// Statements in the phase
    pub body: Vec<Statement>,
}

/// A statement within a phase.
#[derive(Debug, Clone)]
pub enum Statement {
    /// Message send: Role -> Role: Message { ... }
    Send(SendStatement),
    /// Choice block: choice @Role { ... }
    Choice(ChoiceStatement),
    /// Match expression: match Expr { ... }
    Match(MatchStatement),
    /// Continue to another phase
    Continue(ContinueStatement),
    /// End the protocol
    End(Span),
    /// Parallel composition
    Parallel(ParallelStatement),
    /// Reliable channel block
    Reliable(Vec<Statement>),
    /// Unreliable channel block
    Unreliable(Vec<Statement>),
}

/// A message send statement.
#[derive(Debug, Clone)]
pub struct SendStatement {
    /// Source span
    pub span: Span,
    /// Sending role
    pub from: String,
    /// Receiving role
    pub to: String,
    /// Message name
    pub message: String,
    /// Message fields (inline definition)
    pub fields: Vec<Field>,
}

/// A choice statement.
#[derive(Debug, Clone)]
pub struct ChoiceStatement {
    /// Source span
    pub span: Span,
    /// Role making the choice
    pub role: String,
    /// Available branches
    pub branches: Vec<ChoiceBranch>,
}

/// A branch in a choice statement.
#[derive(Debug, Clone)]
pub struct ChoiceBranch {
    /// Branch name
    pub name: String,
    /// Source span
    pub span: Span,
    /// Optional guard condition
    pub guard: Option<Guard>,
    /// Branch body
    pub body: Vec<Statement>,
}

/// A guard condition on a choice branch.
#[derive(Debug, Clone)]
pub struct Guard {
    /// Source span
    pub span: Span,
    /// The role whose state is being checked
    pub role: String,
    /// The condition being checked
    pub condition: String,
}

/// A match statement.
#[derive(Debug, Clone)]
pub struct MatchStatement {
    /// Source span
    pub span: Span,
    /// Expression being matched (e.g., "Message.field")
    pub expr: String,
    /// Match arms
    pub arms: Vec<MatchArm>,
}

/// An arm in a match statement.
#[derive(Debug, Clone)]
pub struct MatchArm {
    /// Pattern to match
    pub pattern: String,
    /// Source span
    pub span: Span,
    /// Arm body
    pub body: Vec<Statement>,
}

/// A continue statement.
#[derive(Debug, Clone)]
pub struct ContinueStatement {
    /// Source span
    pub span: Span,
    /// Target phase name
    pub target: String,
}

/// A parallel composition statement.
#[derive(Debug, Clone)]
pub struct ParallelStatement {
    /// Source span
    pub span: Span,
    /// Parallel branches (phase names or inline blocks)
    pub branches: Vec<String>,
}
