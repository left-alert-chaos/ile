//! # Build
//! This module is for code that reads a source file and builds a corresponding Abstract Syntax
//! Tree.
//! 
//! This includes tokenization and parsing. In order to build and run an AST, you should probably
//! see `ast::ast_from_file()`.

mod parse;
mod token;
pub use parse::*;
pub use token::*;
