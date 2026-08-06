//! # ast
//! This module holds code to represent and build an Abstract Syntax Tree.

pub mod build;
pub mod node;
pub mod scope;

pub use build::*;
pub use node::*;

/// Read a file at the given path and perform the entire module pipeline:
/// - Tokenize the contents
/// - Parse an abstract syntax tree from the tokens
pub fn ast_from_file<'a>(path: impl ToString) -> Result<Node<'a>, String> {
    let path = path.to_string();

    let chars = match std::fs::read(&path) {
        Ok(text) => text,
        Err(_) => return Err(format!("couldn't find module '{path}'")),
    };

    let text = match String::from_utf8(chars) {
        Ok(text) => text,
        Err(_) => return Err(format!("couldn't read module '{path}' as utf8")),
    };

    let tokens = tokenize(text)?;

    Ok(Node::parse(tokens, path)?)
}
