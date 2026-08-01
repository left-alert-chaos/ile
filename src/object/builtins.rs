//! # builtins
//! This module holds object types that come with the interpreter such as `Executable` and `Mod`.
//! When the AST creates an executable block or a Rust library wants to wrap a function, they use
//! `Executable`. Ile-accessible functions are represented with `Object::Function`, which wraps
//! `Executable`.

use super::{Object, DataType};
use crate::Node;

/// # FunctionSignature
/// This is a wrapper around `Vec<Object<'a>>`. It is used to represent the signatures of Ile
/// functions to facilitate interoperability.
pub type FunctionSignature<'a> = Vec<Object<'a>>;

#[derive(Clone)]
pub enum Executable<'a> {
    /// An executable object that is composed of Ile statements (yet to be implemented).
    // TODO
    CodeBlock(&'a Node<'a>),

    /// A wrapper around a Rust function that is callable by Ile code.
    Wrapper {
        signature: FunctionSignature<'a>,
        func: &'a dyn Fn(FunctionSignature<'a>) -> Object,
    },
}

pub struct Mod;

impl<'a> DataType<'a> for Mod {
    fn name(&self) -> String {
        String::from("Mod")
    }

    fn methods(&self) -> std::collections::HashMap<&str, Object<'a>> {
        std::collections::HashMap::new()
    }

    fn attributes(&self) -> std::collections::HashMap<&str, Object<'a>> {
        std::collections::HashMap::new()
    }
}
