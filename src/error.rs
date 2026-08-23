//! # error
//! This module holds logic to display interpreter errors and allow you to create your own. Errors
//! are generated when a piece of source code can't be tokenized, tokens can't be parsed into and
//! AST, or an AST misuses a value or uses a value that doesn't exist.
//!
//! It's also what you use when a function you write doesn't run successfully. Here's an example for
//! an adding library:
//!
//! ```rust
//! use ile::*;
//!
//! // create the library
//! let mut my_lib = module::Library::new("addlib");
//!
//! // add the function to the library to be used by Ile
//! my_lib.add_function(&adder, signature!("int", "int"), "adder");
//!
//! // This adder function will only add integers. If if encounters an argument other than an
//! // integer, it will return an `Error`.
//! fn adder(args: FunctionSignature<'_>) -> FunctionResult<'_> {
//!     if args.len() != 2 {
//!         return Err(Error::new_rust("addlib.adder() only takes 2 arguments"));
//!     }
//!     for arg in &args {
//!         if !arg.is_integer() {
//!             return Err(Error::new_rust("addlib.adder() only takes integers as arguments"));
//!         }
//!     }
//!
//!     Ok(
//!         Some(
//!             Object::Integer(
//!                 args[0].integer().unwrap() + args[1].integer().unwrap()
//!             )
//!         )
//!     )
//! }
//! ```

use std::{error, fmt};

use crate::*;

/// # Error
/// This type represents an issue with tokenization, parsing, or execution. It is often returned
/// inside an `Err(Error)`.
#[derive(Clone)]
pub struct Error {
    pub message: String,
    location: PipelineLocation,
    pub token: Option<Token>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl Error {
    /// Create an `Error` in the parsing pipeline stage with the given message and token
    pub fn new_parsing(token: Option<Token>, message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
            location: PipelineLocation::Parsing,
            token,
        }
    }

    /// Create an `Error` in the walking/executing pipline stage with the given message and token
    pub fn new_runtime(token: Option<Token>, message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
            location: PipelineLocation::Runtime,
            token,
        }
    }

    /// Create an `Error` with an arbitrary message. Used for when something doesn't work in a Rust
    /// function.
    pub fn new_rust(message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
            location: PipelineLocation::Rust,
            token: None,
        }
    }

    /// Create an `Error` with an arbitrary message that represents an error while tokenizing.
    pub fn new_tokenization(message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
            location: PipelineLocation::Tokenization,
            token: None,
        }
    }

    /// Create a detailed error message including location, type, and reason.
    pub fn format(&self) -> String {
        let line;
        let col;
        let location_token;
        let file;
        if let Some(token) = self.token.clone() {
            line = format!("{}", token.line);
            col = format!("{}", token.col);
            location_token = format!("{:?}", token.ttype);
            file = token.file.unwrap_or(String::from("unknown"));
        } else {
            line = String::from("unknown");
            col = String::from("unknown");
            location_token = String::from("unknown");
            file = String::from("unknown");
        }

        format!(
            "{} error at line {}, col {} in file {}:\n{}\nnear token {}",
            self.location, line, col, file, self.message, location_token,
        )
    }
}

impl error::Error for Error {}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        value.format()
    }
}

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

    /// Represents a problem that prevents an AST from successfully walking that is generated in a
    /// Rust function.
    Rust,
}

impl fmt::Display for PipelineLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            PipelineLocation::Tokenization => "tokenization",
            PipelineLocation::Parsing => "parsing",
            PipelineLocation::Runtime => "runtime",
            PipelineLocation::Rust => "rust",
        };

        write!(f, "{repr}")
    }
}
