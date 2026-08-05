//! # error
//! This module holds logic to display interpreter errors and allow you to create your own. Errors
//! are generated when a piece of source code can't be tokenized, tokens can't be parsed into and
//! AST, or an AST misuses a value or uses a value that doesn't exist.

use std::{error, fmt};

/// # Error
/// This type represents an issue with tokenization, parsing, or execution. It is often returned
/// inside an `Err(Error)`.
pub struct Error<'a> {
    message: String,
    location: PipelineLocation,
    file: &'a str,
    line: u32,
    col: u32,
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl fmt::Debug for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl Error<'_> {
    /// Create a detailed error message including location, type, and reason.
    pub fn format(&self) -> String {
        format!(
            "{} error at line {} and col {} in file {}:\n{}",
            self.location, self.line, self.col, self.file, self.message,
        )
    }
}

impl error::Error for Error<'_> {}

#[derive(Clone, Debug)]
/// # PipelineLocation
/// This type represents where in the interpretation pipeline an error occurred.
pub enum PipelineLocation {
    /// Represents a problem that prevents tokenization from continuing.
    Tokenization,

    /// Represents a problem that prevents tokens from being parsed into an Abstract Syntax Tree.
    Parsing,

    /// Represents a problem that prevents an AST from successfully walking.
    Runtime,
}

impl fmt::Display for PipelineLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            PipelineLocation::Tokenization => "tokenization",
            PipelineLocation::Parsing => "parsing",
            PipelineLocation::Runtime => "runtime",
        };

        write!(f, "{repr}")
    }
}
