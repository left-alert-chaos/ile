//! # node
//! This module holds code to represent a node in the Abstract Syntax Tree. This can be anything
//! that can show up in source code--a code block holding other nodes, a function call holding
//! values to pass, or things like numbers, booleans or string declarations.

use crate::FunctionSignature;

pub enum Node<'a> {
    /// Represents a function call
    Call {
        arguments: FunctionSignature<'a>,

        /// This is the full, written path to the function, separated by object; it might look
        /// something like `["object", "attribute", "method"]`.
        path: Vec<String>,
    },
    
    /// Represents modules and functions
    CodeBlock(Vec<Node<'a>>),

    /// Represents a statement. It holds a `Vec` of `Node::Call`s, to allow for method chaining.
    Chain(Vec<Node<'a>>),
}
