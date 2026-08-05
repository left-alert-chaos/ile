//! # parse
//! This module holds code to convert a list of `Token`s into a walkable Abstract Syntax Tree. It's
//! mostly in an `impl` block for `Node`.

use crate::{DataType, FunctionSignature, Node, Token};

impl<'a> Node<'a> {
    /// Parse a `Vec<Token>` into a `Node::Root`
    /// Tokenization happens by storing a parent and child node. The parser creates a child node and
    /// calls the parent's `add_child()` method to appropriately store the new node.
    pub fn parse(tokens: Vec<Token>, fname: String) -> Self {
        let mut root = Self::new_root(fname.clone());

        for token in tokens {}

        root
    }
}
