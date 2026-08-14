//! # builtins
//! This module holds object types that come with the interpreter such as `Executable`.
//! When the AST creates an executable block or a Rust library wants to wrap a function, they use
//! `Executable`. Ile-accessible functions are represented with `Object::Function`, which wraps
//! `Executable`.

use super::Object;
use crate::*;

use std::fmt;

/// # FunctionSignature
/// This is a wrapper around `Vec<Object<'a>>`. It is used to represent the signatures of Ile
/// functions to facilitate interoperability.
pub type FunctionSignature<'a> = Vec<Object<'a>>;

#[derive(Clone)]
pub enum Executable<'a> {
    /// An executable object that is composed of Ile statements.
    CodeBlock(Box<Node<'a>>),

    /// A wrapper around a Rust function that is callable by Ile code.
    Wrapper {
        signature: FunctionSignature<'a>,
        func: &'a dyn Fn(FunctionSignature<'a>) -> Result<Option<Object<'a>>, error::Error>,
    },
}

impl fmt::Debug for Executable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CodeBlock(n) => write!(f, "{n:?}"),
            Self::Wrapper { signature, .. } => write!(f, "Executable::Wrapper with signature {}", debug_signature(signature)),
        }
    }
}

fn debug_signature(s: &FunctionSignature<'_>) -> String {
    let mut res = String::new();

    for object in s {
        res.push_str(format!("{object:?} ").as_str());
    }

    res
}
