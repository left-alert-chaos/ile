//! # node
//! This module holds code to represent a node in the Abstract Syntax Tree. This can be anything
//! that can show up in source code--a code block holding other nodes, a function call holding
//! values to pass, or things like numbers, booleans or string declarations.

use crate::{
    FunctionSignature,
    Object,
    DataType,
};
use super::scope::ScopeStack;

/// # Node
/// A node is any part of an AST.
pub enum Node<'a> {
    /// Represents a function call
    Call {
        arguments: FunctionSignature<'a>,

        /// This is the full, written path to the function, separated by object; it might look
        /// something like `["object", "attribute", "method"]`.
        path: Vec<String>,
    },
    
    /// Represents functions
    CodeBlock(Vec<Node<'a>>),

    /// Represents a statement. It holds a `Vec` of `Node::Call`s, to allow for method chaining.
    Chain(Vec<Node<'a>>),

    /// Represents assigning a value to a named variable.
    Assignment {
        name: String,
        value: Object<'a>,
    },

    /// Represents a `DataType` definition.
    DataType(DataType<'a>),
    
    /// Represents the root of one module. Modules can hold others.
    Root {
        stack: ScopeStack<'a>,
        imports: Vec<Node<'a>>,
        types: Vec<DataType<'a>>,
    },
}

impl Node<'_> {
    /// Create a new `Node::Root`
    pub fn new_root() -> Self {
        Self::Root {
            stack: ScopeStack::new(),
            imports: Vec::new(),
            types: Vec::new(),
        }
    }
}
