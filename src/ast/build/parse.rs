//! # parse
//! This module holds code to convert a list of `Token`s into a walkable Abstract Syntax Tree. It's
//! mostly in an `impl` block for `Node`.

use crate::{DataType, FunctionSignature, Node, Token, TokenType, error::Error};
use core::slice::Iter;

impl<'a> Node<'a> {
    /// Parse a `Vec<Token>` into a `Node::Root`. This is the main representation of an AST.
    /// Nesting happens by storing a parent and child node. The parser creates a child node and
    /// calls the parent's `add_child()` method to appropriately store the new node.
    pub fn build_root(tokens: Vec<Token>, fname: String) -> Result<Self, Error> {
        let mut root = Self::new_root(fname.clone());

        for token in tokens.iter() {
            match token.ttype {
                // handle opening a code
                TokenType::OpenBrace => {
                    
                }
                // end the current node
                TokenType::ChainEnd => {
                }
                _ => {}
            }
        }

        Ok(root)
    }

    // Responsible for creating a node and recursing to create children
    fn parse_individual_node(iterator: &mut Iter<Token>, fname: String) -> Result<Node<'a>, Error> {
        //Extract first token's info
        let Some(token) = iterator.next() else {
            return Err(Error::new_parsing(None, "unexpected EOF", fname.as_str()));
        };
        let TokenType::Word(word) = token.ttype.clone() else {
            return Err(Error::new_parsing(Some(*token), "expected word", fname.as_str()));
        };

        // determine node type from first token
        match word.as_str() {
            "let" => Self::parse_let(iterator, fname),
            _ => Err(String::new())
        }
    }

    fn parse_let(iterator: &mut Iter<Token>, fname: String) -> Result<Node<'a>, Error> {
        let word = Self::expect_word(iterator, fname.clone(), "expected variable name")?;

        // check if there's an equals sign
        Self::expect_single_char(iterator, TokenType::Assignment, fname.clone(), format!("while parsing let statement"))?;
    }

    /// Return the `String` of the next token if it is a `Word`. Otherwise, create an error with
    /// specified message
    fn expect_word(iterator: &mut Iter<Token>, fname: String, message: &str) -> Result<String, Error> {
        let Some(word) = iterator.next() else {
            return Err(Error::new_parsing(None, "unexpected EOF (expected word token)", fname));
        };
        let TokenType::Word(word) = word.ttype.clone() else {
            return Err(Error::new_parsing(Some(word.clone()), format!("{message} (Word token);\nfound {}", word.ttype), fname))
        };

        Ok(word)
    }

    /// Return a `Result<(), Error>` about whether the specified token type is next. Most useful for
    /// asserting that a single-character token is next.
    fn expect_single_char(iterator: &mut Iter<Token>, token_type: TokenType, fname: String, message: impl ToString) -> Result<(), Error> {
        let message = message.to_string();

        let Some(token) = iterator.next() else {
            return Err(Error::new_parsing(None, format!("unexpected EOF (expected {token_type} token)"), fname));
        };
        if token.ttype != token_type {
            Err(Error::new_parsing(Some(token.clone()), format!("expected {token_type} token {message};\nfound {}", token.ttype), fname))
        } else {
            Ok(())
        }
    }
}
