//! # builtins
//! This module holds object types that come with the interpreter such as `Executable`.
//! When the AST creates an executable block or a Rust library wants to wrap a function, they use
//! `Executable`. Ile-accessible functions are represented with `Object::Function`, which wraps
//! `Executable`.

use super::Object;
use crate::Node;

/// # FunctionSignature
/// This is a wrapper around `Vec<Object<'a>>`. It is used to represent the signatures of Ile
/// functions to facilitate interoperability.
pub type FunctionSignature<'a> = Vec<Object<'a>>;

#[derive(Clone)]
pub enum Executable<'a> {
    /// An executable object that is composed of Ile statements.
    CodeBlock(&'a Node<'a>),

    /// A wrapper around a Rust function that is callable by Ile code.
    Wrapper {
        signature: FunctionSignature<'a>,
        func: &'a dyn Fn(FunctionSignature<'a>) -> Object,
    },
}
