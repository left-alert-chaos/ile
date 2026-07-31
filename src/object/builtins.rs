//! # builtins
//! This module holds object types that come with the interpreter such as `Executable` and `Mod`.
//! When the AST creates an executable block or a Rust library wants to wrap a function, they use
//! `Executable`.

use super::{Object, DataType};

/// # FunctionSignature
/// This is a wrapper around `Vec<Object<'a>>`. It is used to represent the signatures of Ile
/// functions to facilitate interoperability.
pub type FunctionSignature<'a> = Vec<Object<'a>>;

#[derive(Clone)]
pub enum Executable<'a> {
    /// An executable object that is composed of Ile statements (yet to be implemented).
    CodeBlock,

    /// A wrapper around a Rust function that is callable by Ile code.
    Wrapper {
        signature: FunctionSignature<'a>,
        func: &'a Box<dyn Fn(FunctionSignature<'a>) -> Object>,
    },
}
