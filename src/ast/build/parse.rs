//! # parse
//! This module holds code to convert a list of `Token`s into a walkable Abstract Syntax Tree. It's
//! mostly in an `impl` block for `Node`.

use crate::{DataType, FunctionSignature, Node, Token, error::Error};

impl<'a> Node<'a> {
    /// Parse a `Vec<Token>` into a `Node::Root`
    /// Tokenization happens by storing a parent and child node. The parser creates a child node and
    /// calls the parent's `add_child()` method to appropriately store the new node.
    pub fn parse(tokens: Vec<Token>, fname: String) -> Result<Self, Error<'a>> {
        let mut root = Self::new_root(fname.clone());
        let mut parent = &mut root;

        for token in tokens {
            
        }

        Ok(root)
    }
}
