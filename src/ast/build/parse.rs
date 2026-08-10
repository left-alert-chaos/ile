//! # parse
//! This module holds code to convert a list of `Token`s into a walkable Abstract Syntax Tree. It's
//! mostly in an `impl` block for `Node`.

use crate::{DataType, FunctionSignature, Node, Token, TokenType, error::Error};
use core::slice::Iter;

/// # Parser
pub struct Parser {
    index: usize,
    tokens: Vec<Token>,
}

impl<'a> Parser {
    /// Parse a `Vec<Token>` into a `Node::Root`. This is the main representation of an AST.
    /// Nesting happens by storing a parent and child node. The parser creates a child node and
    /// calls the parent's `add_child()` method to appropriately store the new node.
    pub fn build_root(tokens: Vec<Token>, fname: String) -> Result<Node<'a>, Error> {
        let mut root = Node::new_root(fname.clone());

        let mut parser = Self {
            tokens,
            index: 0,
        };

        while let Some(_) = parser.current() {
            root.root_add_child(parser.parse_individual_node(fname.clone())?);
        }

        Ok(root)
    }

    // Responsible for creating a node and recursing to create children
    fn parse_individual_node(&mut self, fname: String) -> Result<Node<'a>, Error> {
        //Extract first token's info
        let Some(token) = self.next() else {
            return Err(Error::new_parsing(None, "unexpected EOF", fname.as_str()));
        };
        let TokenType::Word(word) = token.ttype.clone() else {
            return Err(Error::new_parsing(
                Some(token.clone()),
                "expected word",
                fname.as_str(),
            ));
        };

        // determine node type from first token
        match word.as_str() {
            "let" => self.parse_let(fname),
            _ => self.parse_misc(word, fname),
        }
    }

    fn parse_let(&mut self, fname: String) -> Result<Node<'a>, Error> {
        let name = self.expect_word(fname.clone(), "expected variable name")?;

        // check if there's an equals sign
        self.expect_single_char(
            TokenType::Assignment,
            fname.clone(),
            "while parsing let statement",
        )?;

        let value = self.parse_individual_node(fname.clone())?;

        // check for semicolon
        self.expect_single_char(
            TokenType::ChainEnd,
            fname,
            "while parsing let statement",
        )?;

        Ok(
            Node::Assignment {
                path: Vec::from([name]),
                value: Box::new(value),
                create: true
            }
        )
    }

    fn parse_assignment(&mut self, path: Vec<String>, fname: String) -> Result<Node<'a>, Error> {
        let value = self.parse_individual_node(fname.clone())?;
        self.expect_single_char(
            TokenType::ChainEnd,
            fname,
            "while parsing assignment",
        )?;
        
        Ok(
            Node::Assignment {
                path,
                value: Box::new(value),
                create: true,
            }
        )
    }

    fn parse_call(&mut self, fname: String, path: Vec<String>) -> Result<Node<'a>, Error> {
        let mut children = Vec::new();

        loop {
            children.push(self.parse_individual_node(fname.clone())?);

            // check previous token to determine if the parens ended or its a comma
            let Some(prev) = self.peek_prev() else {
                return Err(Error::new_parsing(None, "unexpected EOF while parsing function call", fname));
            };

            if prev.ttype == TokenType::CloseParen {
                break;
            }
        }

        Ok(
            Node::Call {
                arguments: children,
                path,
            }
        )
    }

    /// Parse a non-keyword
    fn parse_misc(&mut self, word: String, fname: String) -> Result<Node<'a>, Error> {
        let Some(next) = self.next() else {
            return Err(Error::new_parsing(
                None,
                "unexpected EOF while parsing statement",
                fname.clone()
            ));
        };

        // non-keywords are always paths to something else, so read the path
        let mut path = Vec::from([word]);
        let mut chain = Vec::new();
        while let Some(token) = self.next() {
            match token.ttype.clone() {
                TokenType::PathSeparator => {},
                TokenType::Word(w) => path.push(w),
                TokenType::OpenParen => chain.push(self.parse_call(fname.clone(), path.clone())?),
                TokenType::Assignment => return self.parse_assignment(path, fname.clone()),
                TokenType::ChainEnd => break,
                // push an operator or raise an error
                _ => {
                    if token.ttype.is_operator() {
                        chain.push(Node::Variable(path.clone()));
                        path.clear(); //paving the way lol
                        chain.push(Node::Operator(token.ttype.clone()));
                    } else {
                        return Err(
                            Error::new_parsing(Some(token.clone()), format!("unexpected token type {}", token.ttype), fname)
                        )
                    }
                }
            }
        }

        Ok(
            Node::Chain(chain)
        )
    }

    /// Return the `String` of the next token if it is a `Word`. Otherwise, create an error with
    /// specified message
    fn expect_word(
        &mut self,
        fname: String,
        message: &str,
    ) -> Result<String, Error> {
        let Some(word) = self.next() else {
            return Err(Error::new_parsing(
                None,
                "unexpected EOF (expected word token)",
                fname,
            ));
        };
        let TokenType::Word(word) = word.ttype.clone() else {
            return Err(Error::new_parsing(
                Some(word.clone()),
                format!("{message} (Word token);\nfound {}", word.ttype),
                fname,
            ));
        };

        Ok(word)
    }

    /// Return a `Result<(), Error>` about whether the specified token type is next. Most useful for
    /// asserting that a single-character token is next.
    fn expect_single_char(
        &mut self,
        token_type: TokenType,
        fname: String,
        message: impl ToString,
    ) -> Result<(), Error> {
        let message = message.to_string();

        let Some(token) = self.next() else {
            return Err(Error::new_parsing(
                None,
                format!("unexpected EOF (expected {token_type} token) {message}"),
                fname,
            ));
        };
        if token.ttype != token_type {
            Err(Error::new_parsing(
                Some(token.clone()),
                format!(
                    "expected {token_type} token {message};\nfound {}",
                    token.ttype
                ),
                fname,
            ))
        } else {
            Ok(())
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.index += 1;
        if self.index < self.tokens.len() {
            Some(self.tokens[self.index].clone())
        } else {
            None
        }
    }

    fn peek_next(&self) -> Option<Token> {
        let index = self.index + 1;
        if self.index < self.tokens.len() {
            Some(self.tokens[index].clone())
        } else {
            None
        }
    }

    fn peek_prev(&self) -> Option<Token> {
        let index = self.index - 1;
        if index > 0 && index < self.tokens.len() {
            Some(self.tokens[index].clone())
        } else {
            None
        }
    }

    fn current(&self) -> Option<Token> {
        if self.index > 0 && self.index < self.tokens.len() {
            Some(self.tokens[self.index].clone())
        } else {
            None
        }
    }
}
