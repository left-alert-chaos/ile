//! # parse
//! This module holds code to convert a list of `Token`s into a walkable Abstract Syntax Tree. It's
//! mostly in an `impl` block for `Node`.

use crate::{
    Node,
    FunctionSignature,
    DataType,
    Token,
};

impl<'a> Node<'a> {
    /// Parse a `Vec<Token>` into a `Node::Root`
    pub fn parse(tokens: Vec<Token>) -> Self {
        let mut root = Self::new_root();



        root
    }
}
