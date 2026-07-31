//! # token
//! This module is for initial parsing of a source file--it takes raw code and converts it into a
//! `Vector` of `Tokens` which represent symbols and syntax. These symbols are then used to build an
//! AST. This is the first step of interpretation.

/// # Token
/// This is a non-public type representing a processed token of source code. It isn't necessary for
/// using the interpreter and is only an intermediate step from source to AST.
#[derive(Debug)]
pub enum Token {
    /// A token for (
    OpenParen,

    /// A token for )
    CloseParen,

    /// A token for [
    OpenBracket,

    /// A token for ]
    CloseBracket,

    /// A token for {
    OpenBrace,

    /// A token for }
    CloseBrace,

    /// A token for a path separator or "dot"
    PathSeparator,

    /// A token for the end of a statement, which is a semicolon (;)
    ChainEnd,

    /// A token for the assignment operator or single equals sign (=)
    Assignment,

    /// A token for the less than operator (<)
    LessThan,

    /// A token for the greater than operator (>)
    GreaterThan,

    /// A token for the addition operator (+)
    Addition,

    /// A token for the subtraction operator (-)
    Subtraction,

    /// A token for the multiplication operator, in this case an asterisk (*)
    Multiplication,

    /// A token for the division operator, in this case a forward-slash (/)
    Division,

    /// A token for anything that doesn't fit into any other bucket; it's usually object and type
    /// names.
    Word(String),
}
