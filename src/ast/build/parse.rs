//! # parse
//! This module holds code to convert a list of `Token`s into a walkable Abstract Syntax Tree. It's
//! mostly in an `impl` block for `Node`.

use crate::{Node, NodeType, Object, Token, TokenType, error::Error};

use std::collections::HashMap;

/// # Parser
pub struct Parser {
    index: usize,
    tokens: Vec<Token>,
    started: bool,
}

impl<'a> Parser {
    /// Parse a `Vec<Token>` into a `Node::Root`. This is the main representation of an AST.
    /// Nesting happens by storing a parent and child node. The parser creates a child node and
    /// calls the parent's `add_child()` method to appropriately store the new node.
    pub fn build_root(tokens: Vec<Token>, fname: String) -> Result<Node<'a>, Error> {
        let mut root = Node::new_root(fname);

        let mut parser = Self {
            tokens,
            index: 0,
            started: false,
        };

        while parser.peek_next().is_some() {
            let child = parser.parse_individual_node()?;
            root.root_add_child(child);
        }

        Ok(root)
    }

    // Responsible for creating a node and recursing to create children
    fn parse_individual_node(&mut self) -> Result<Node<'a>, Error> {
        //Extract first token's info
        let Some(token) = self.next() else {
            return Err(Error::new_parsing(None, "unexpected EOF"));
        };

        match token.ttype {
            TokenType::OpenParen => return self.parse_function(),
            TokenType::OpenBrace => return self.parse_block(),
            TokenType::OpenBracket => return self.parse_array(),
            _ => {}
        }

        // literals
        if let Ok(obj) = Object::from_token(token.clone()) {
            if let Some(next) = self.peek_next()
                && next.ttype.is_operator()
            {
                self.index -= 1;
                return self.parse_misc(None);
            } else {
                return Ok(Node {
                    ntype: NodeType::Literal(obj),
                    token: Some(token),
                });
            }
        }

        // if the things before this didn't work, this can only be a word
        let TokenType::Word(word) = token.ttype.clone() else {
            return Err(Error::new_parsing(Some(token.clone()), "expected word"));
        };

        // determine node type from first token
        match word.as_str() {
            "let" => self.parse_let(),
            "if" => self.parse_if(),
            "for" => self.parse_for(),
            "while" => self.parse_while(),
            "datatype" => self.parse_datatype(),
            "import" => self.parse_import(),
            "return" => self.parse_return(),
            "break" | "continue" => self.parse_control_flow(),
            _ => self.parse_misc(Some(word)),
        }
    }

    /// Returns a `Break` or `Continue`
    fn parse_control_flow(&mut self) -> Result<Node<'a>, Error> {
        self.index -= 1;
        let Token {
            ttype: TokenType::Word(word),
            ..
        } = self.next().unwrap()
        else {
            unreachable!();
        };
        let ntype = match word.as_str() {
            "break" => NodeType::Break,
            "continue" => NodeType::Continue,
            _ => unreachable!(),
        };
        if let Some(next) = self.peek_next()
            && next.ttype == TokenType::ChainEnd
        {
            self.index += 1;
        }
        Ok(Node {
            token: self.current(),
            ntype,
        })
    }

    fn parse_return(&mut self) -> Result<Node<'a>, Error> {
        let value = self.parse_individual_node()?;
        self.expect_single_char(TokenType::ChainEnd, "while finishing return statement")?;
        Ok(Node {
            token: self.current(),
            ntype: NodeType::Return(Box::new(value)),
        })
    }

    // called after unexpected open paren that isn't after a path
    fn parse_function(&mut self) -> Result<Node<'a>, Error> {
        // parse the signature
        let mut signature = Vec::new();
        while let Some(token) = self.next() {
            match token.ttype {
                TokenType::Word(w) => signature.push(w),
                TokenType::Comma => {}
                TokenType::CloseParen => break,
                _ => {
                    return Err(Error::new_parsing(
                        Some(token.clone()),
                        format!(
                            "unexpected {} token while parsing function signature; expected Comma, CloseParen, or Word",
                            token.ttype
                        ),
                    ));
                }
            }
        }

        // parse block
        // the first match arm is a node holding a nodetype::codeblock and the only extrracted value
        // is chains
        self.expect_single_char(TokenType::OpenBrace, "while parsing function definition")?;
        match self.parse_block() {
            Ok(Node {
                ntype: NodeType::CodeBlock { chains, .. },
                ..
            }) => Ok(Node {
                ntype: NodeType::CodeBlock { chains, signature },
                token: self.current(),
            }),
            Err(e) => Err(e),
            _ => unreachable!(),
        }
    }

    /// parse an if statement, including and else
    fn parse_if(&mut self) -> Result<Node<'a>, Error> {
        let condition = self.parse_individual_node()?;
        self.expect_single_char(
            TokenType::OpenBrace,
            "to open block while parsing if statement",
        )?;
        let block = self.parse_block()?;

        // else clause?
        let else_clause = if let Some(next) = self.peek_next()
            && next.ttype == TokenType::Word(String::from("else"))
        {
            self.index += 1;
            Some(Box::new(self.parse_individual_node()?))
        } else {
            None
        };

        Ok(Node {
            ntype: NodeType::If {
                condition: Box::new(condition),
                block: Box::new(block),
                else_clause,
            },
            token: self.current(),
        })
    }

    // parse a for loop
    // The logic is almost identical to the if statement, just there aren't any else clauses to
    // worry about
    fn parse_for(&mut self) -> Result<Node<'a>, Error> {
        let condition = self.parse_individual_node()?;
        self.expect_single_char(TokenType::OpenBrace, "to open block while parsing for loop")?;
        let block = self.parse_block()?;

        Ok(Node {
            ntype: NodeType::For {
                condition: Box::new(condition),
                block: Box::new(block),
            },
            token: self.current(),
        })
    }

    // parse a while loop
    // This logic is again similar to the logic for `for` and `if`
    fn parse_while(&mut self) -> Result<Node<'a>, Error> {
        let condition = self.parse_individual_node()?;
        self.expect_single_char(
            TokenType::OpenBrace,
            "to open block while parsing while loop",
        )?;
        let block = self.parse_block()?;

        Ok(Node {
            ntype: NodeType::While {
                condition: Box::new(condition),
                block: Box::new(block),
            },
            token: self.current(),
        })
    }

    // parse child nodes until a CloseBrace is reached
    fn parse_block(&mut self) -> Result<Node<'a>, Error> {
        let mut chains = Vec::new();

        while let Some(token) = self.peek_next()
            && token.ttype != TokenType::CloseBrace
        {
            chains.push(self.parse_individual_node()?);
        }

        // consume CloseBrace or EOF?
        match self.peek_next() {
            Some(_) => self.index += 1,
            None => {
                return Err(Error::new_parsing(
                    None,
                    "unexpected EOF while parsing block",
                ));
            }
        }

        Ok(Node {
            ntype: NodeType::CodeBlock {
                chains,
                signature: Vec::new(),
            },
            token: self.current(),
        })
    }

    // parse an array declaration (stuff in [])
    fn parse_array(&mut self) -> Result<Node<'a>, Error> {
        let mut children = Vec::new();

        while let Some(token) = self.peek_next()
            && token.ttype != TokenType::CloseBracket
        {
            children.push(self.parse_individual_node()?);

            // consume comma if there is one
            if let Some(token) = self.peek_next()
                && token.ttype == TokenType::Comma
            {
                self.index += 1;
            }
        }

        // consume CloseBracket
        self.expect_single_char(TokenType::CloseBracket, "while ending array literal")?;

        Ok(Node {
            token: self.current(),
            ntype: NodeType::ArrayLiteral(children),
        })
    }

    // parse let statements inside a `datatype` block
    fn parse_datatype(&mut self) -> Result<Node<'a>, Error> {
        let mut attributes = HashMap::new();

        // get name
        let Some(next) = self.next() else {
            return Err(Error::new_parsing(
                None,
                "unexpected EOF while parsing datatype",
            ));
        };
        let TokenType::Word(name) = next.ttype else {
            return Err(Error::new_parsing(
                Some(next.clone()),
                format!(
                    "expected Word while parsing datatype name;\nfound {}",
                    next.ttype
                ),
            ));
        };

        self.expect_single_char(TokenType::OpenBrace, "while parsing datatype definition")?;

        // read all let statements
        while let Some(token) = self.peek_next()
            && token.ttype != TokenType::CloseBrace
        {
            let assignment = self.parse_individual_node()?;

            let NodeType::Assignment {
                path,
                value,
                create,
            } = assignment.ntype
            else {
                return Err(Error::new_parsing(
                    self.current(),
                    "only let statements are allowed inside datatype definitions",
                ));
            };

            if !create {
                return Err(Error::new_parsing(
                    self.current(),
                    "only let statements are allowed inside datatype definitions. Help: add let",
                ));
            }

            if path.len() != 1 {
                return Err(Error::new_parsing(
                    self.current(),
                    "only local assignments are allowed inside datatype definitions",
                ));
            }

            // determine where to put value
            match value.ntype {
                NodeType::Literal(_)
                | NodeType::Call { .. }
                | NodeType::CodeBlock { .. }
                | NodeType::Chain(_) => {
                    attributes.insert(path[0].clone(), *value);
                }
                _ => {
                    return Err(Error::new_parsing(
                        self.current(),
                        format!(
                            "only functions, calls, literals and chains can be assigned inside datatype definitions, not {:?}",
                            value.ntype
                        ),
                    ));
                }
            }
        }

        self.expect_single_char(
            TokenType::CloseBrace,
            "while parsing end of datatype definition",
        )?;

        Ok(Node {
            ntype: NodeType::DataType { name, attributes },
            token: self.current(),
        })
    }

    fn parse_let(&mut self) -> Result<Node<'a>, Error> {
        let name = self.expect_word("expected variable name")?;

        // check if there's an equals sign
        self.expect_single_char(TokenType::Assignment, "while parsing let statement")?;

        let value = self.parse_individual_node()?;

        // check for semicolon
        if self.current().unwrap().ttype != TokenType::ChainEnd {
            self.expect_single_char(TokenType::ChainEnd, "while parsing let statement")?;
        }

        Ok(Node {
            ntype: NodeType::Assignment {
                path: Vec::from([name]),
                value: Box::new(value),
                create: true,
            },
            token: self.current(),
        })
    }

    fn parse_assignment(&mut self, path: Vec<String>) -> Result<Node<'a>, Error> {
        let value = self.parse_individual_node()?;
        self.expect_single_char(TokenType::ChainEnd, "while parsing assignment")?;

        Ok(Node {
            ntype: NodeType::Assignment {
                path,
                value: Box::new(value),
                create: false,
            },
            token: self.current(),
        })
    }

    fn parse_call(&mut self, path: Vec<String>) -> Result<Node<'a>, Error> {
        let mut children = Vec::new();

        loop {
            // support empty calls
            if let Some(next) = self.peek_next()
                && next.ttype == TokenType::CloseParen
            {
                self.index += 1;
                break;
            }

            children.push(self.parse_individual_node()?);


            // check next token to determine if the parens ended or its a comma
            if let Some(next) = self.peek_next() {
                if next.ttype == TokenType::CloseParen || next.ttype == TokenType::Comma {
                    self.index += 1;
                }
                if next.ttype == TokenType::CloseParen {
                    break;
                }
            }
        }

        // if next char is semicolon, consume
        if let Some(token) = self.peek_next()
            && token.ttype == TokenType::ChainEnd
        {
            self.index += 1;
        }

        Ok(Node {
            ntype: NodeType::Call {
                arguments: children,
                path,
            },
            token: self.current(),
        })
    }

    /// Responsible for parsing comparisons and operators
    // The chain is the expression before it was determined to be an operator
    fn parse_operator(&mut self, chain: Vec<Node<'a>>) -> Result<Node<'a>, Error> {
        let mut token = self.current().unwrap();

        // create chain node
        let mut first_arm = Node {
            token: Some(token.clone()),
            ntype: NodeType::Chain(chain),
        };

        let mut second_expression = self.parse_individual_node()?;

        // change child operator if it takes precedence (no way that's spelled right)
        if let NodeType::Operator(second_operator, second_operator_arm1, second_operator_arm2) =
            second_expression.clone().ntype
            && (second_operator.is_boolean_operator() && !token.clone().ttype.is_boolean_operator())
        {
            // move the previous operator into the first arm, as well as the first arm of the second
            // operator
            let new_first_arm = Node {
                token: Some(token.clone()),
                ntype: NodeType::Operator(
                    token.clone().ttype,
                    Box::new(first_arm.clone()),
                    second_operator_arm1,
                ),
            };
            first_arm = new_first_arm;

            // take the token from the second operator and expand it
            token = second_expression.clone().token.unwrap();
            second_expression = *second_operator_arm2;
        }

        Ok(Node {
            token: Some(token.clone()),
            ntype: NodeType::Operator(
                token.ttype,
                Box::new(first_arm),
                Box::new(second_expression),
            ),
        })
    }

    /// Parse indexing an array with square brackets
    fn parse_index(&mut self, path: &Vec<String>) -> Result<Node<'a>, Error> {
        let num1 = self.parse_individual_node()?;

        let mut num2 = None;
        if let Some(next) = self.peek_next() && next.ttype == TokenType::Comma {
            self.index += 1;
            num2 = Some(Box::new(self.parse_individual_node()?));
        }

        self.expect_single_char(TokenType::CloseBracket, "while parsing end of index")?;

        Ok(
            Node {
                token: self.current(),
                ntype: NodeType::Index {
                    path: path.clone(),
                    index1: Box::new(num1),
                    index2: num2,
                }
            }
        )
    }

    /// Parse a non-keyword
    fn parse_misc(&mut self, word: Option<String>) -> Result<Node<'a>, Error> {
        // non-keywords are always paths to something else, so read the path
        let mut path = Vec::new();

        if let Some(word) = word {
            path.push(word);
        }

        let mut chain = Vec::new();
        while let Some(token) = self.next() {
            match token.ttype.clone() {
                TokenType::PathSeparator => {}
                TokenType::Word(w) => path.push(w),
                TokenType::OpenParen => {
                    chain.push(self.parse_call(path.clone())?);

                    // if the call consumed a semicolon, break
                    if let Some(current) = self.current()
                        && current.ttype == TokenType::ChainEnd
                    {
                        break;
                    }

                    path.clear();
                }
                TokenType::OpenBracket => {
                    chain.push(self.parse_index(&path)?);
                    path = Vec::new();
                }
                TokenType::Assignment => return self.parse_assignment(path),
                TokenType::CloseParen
                | TokenType::CloseBrace
                | TokenType::OpenBrace
                | TokenType::ChainEnd
                | TokenType::Comma => {
                    if !path.is_empty() {
                        chain.push(Node {
                            ntype: NodeType::Variable(path.clone()),
                            token: Some(token.clone()),
                        });
                    }
                    self.index -= 1;
                    break;
                }
                // push an operator or literal or raise an error
                _ => {
                    if token.ttype.is_operator() {
                        if !path.is_empty() {
                            chain.push(Node {
                                ntype: NodeType::Variable(path.clone()),
                                token: Some(token.clone()),
                            });
                            path.clear(); //paving the way lol
                        }

                        //chain.push(Node { ntype: NodeType::Operator(token.ttype.clone()), token: Some(token) });
                        return self.parse_operator(chain);
                    } else if let Ok(obj) = Object::from_token(token.clone()) {
                        chain.push(Node {
                            ntype: NodeType::Literal(obj),
                            token: Some(token),
                        });
                    } else {
                        return Err(Error::new_parsing(
                            Some(token.clone()),
                            format!("unexpected token type {}", token.ttype),
                        ));
                    }
                }
            }
        }

        // first check if it's a lone call or lookup
        if chain.len() == 1 {
            Ok(chain[0].clone())
        } else {
            Ok(Node {
                ntype: NodeType::Chain(chain),
                token: self.current(),
            })
        }
    }

    fn parse_import(&mut self) -> Result<Node<'a>, Error> {
        let Some(token) = self.next() else {
            return Err(Error::new_parsing(
                None,
                "unexpected EOF while parsing import",
            ));
        };
        let TokenType::String(modname) = token.clone().ttype else {
            return Err(Error::new_parsing(
                Some(token.clone()),
                format!("import names must be Strings, not {}", token.clone().ttype).as_str(),
            ));
        };

        self.expect_single_char(TokenType::ChainEnd, "while parsing import")?;

        Ok(Node {
            ntype: NodeType::Import(modname),
            token: Some(token),
        })
    }

    /// Return the `String` of the next token if it is a `Word`. Otherwise, create an error with
    /// specified message
    fn expect_word(&mut self, message: &str) -> Result<String, Error> {
        let Some(word) = self.next() else {
            return Err(Error::new_parsing(
                None,
                "unexpected EOF (expected word token)",
            ));
        };
        let TokenType::Word(word) = word.ttype.clone() else {
            return Err(Error::new_parsing(
                Some(word.clone()),
                format!("{message} (Word token);\nfound {}", word.ttype),
            ));
        };

        Ok(word)
    }

    /// Return a `Result<(), Error>` about whether the specified token type is next. Most useful for
    /// asserting that a single-character token is next.
    fn expect_single_char(
        &mut self,
        token_type: TokenType,
        message: impl ToString,
    ) -> Result<(), Error> {
        let message = message.to_string();

        let Some(token) = self.next() else {
            return Err(Error::new_parsing(
                None,
                format!("unexpected EOF (expected {token_type} token) {message}"),
            ));
        };
        if token.ttype != token_type {
            Err(Error::new_parsing(
                Some(token.clone()),
                format!(
                    "expected {token_type} token {message};\nfound {}",
                    token.ttype
                ),
            ))
        } else {
            Ok(())
        }
    }

    fn next(&mut self) -> Option<Token> {
        // so that the 0-index token is used
        if !self.started {
            self.started = true;

            if !self.tokens.is_empty() {
                return Some(self.tokens[0].clone());
            } else {
                return None;
            }
        }

        self.index += 1;
        if self.index < self.tokens.len() {
            Some(self.tokens[self.index].clone())
        } else {
            None
        }
    }

    fn peek_next(&self) -> Option<Token> {
        // make sure to use the 0-index token
        if !self.started {
            if !self.tokens.is_empty() {
                return Some(self.tokens[0].clone());
            } else {
                return None;
            }
        }

        let index = self.index + 1;
        if index < self.tokens.len() {
            Some(self.tokens[index].clone())
        } else {
            None
        }
    }

    fn current(&self) -> Option<Token> {
        if self.index < self.tokens.len() {
            Some(self.tokens[self.index].clone())
        } else {
            None
        }
    }
}
