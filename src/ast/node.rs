//! # node
//! This module holds code to represent a node in the Abstract Syntax Tree. This can be anything
//! that can show up in source code--a code block holding other nodes, a function call holding
//! values to pass, or things like numbers, booleans or string declarations.

use super::scope::ScopeStack;
use crate::*;

use std::collections::HashMap;

/// # Path
/// This is an alias for `Vec<String>` and used to represent a path to an object.
type Path = Vec<String>;

/// # Children<'a>
/// This is an alias for `Vec<Node<'a>Type<'a>>`, used to represent a `Node`'s children.
type Children<'a> = Vec<Node<'a>>;

/// # Node<'a>Type
/// A node is any part of an AST.
#[derive(Clone, Debug)]
pub enum NodeType<'a> {
    /// Represents a function call
    Call {
        arguments: Children<'a>,

        /// This is the full, written path to the function, separated by object; it might look
        /// something like `["object", "attribute", "method"]`.
        path: Path,
    },

    /// Represents a `return` statement
    Return(Option<Box<Node<'a>>>),

    /// Represents a `break` statement
    Break,

    /// Represents a `continue` statement
    Continue,

    /// Represents a logic operator (addition, subtraction, comparisons, etc)
    Operator(TokenType, Box<Node<'a>>, Box<Node<'a>>),

    /// Represents functions
    CodeBlock {
        chains: Children<'a>, // children should be Chain
        signature: Vec<String>,
    },

    /// Represents a statement. It holds a `Vec` of `Node<'a>Type::Call`, `Node::Variable`, or
    /// `Node<'a>Type::Literal`s to allow for method chaining.
    Chain(Children<'a>), // children should be Call or Literal or Variable

    /// Represents assigning a value to a named variable.
    Assignment {
        path: Path,
        value: Box<Node<'a>>, // child should be Chain

        /// Represents whether this is a `let` statement (creating a variable) or a reassignment
        create: bool,
    },

    /// Represents a `DataType` definition.
    /// The read `DataType` type isn't used here because attributes can come from functions, which
    /// are only usable at runtime
    DataType {
        name: String,
        attributes: HashMap<String, Node<'a>>,
    },

    /// Represents a `while` loop.
    While {
        condition: Box<Node<'a>>, // should be Chain
        block: Box<Node<'a>>,     // should be a CodeBlock
    },

    /// Represents a `for` loop.
    For {
        condition: Box<Node<'a>>, // should be anything executable
        block: Box<Node<'a>>,     // should be a CodeBlock
    },

    /// Represents an `if` block.
    If {
        condition: Box<Node<'a>>,           // should be Chain
        block: Box<Node<'a>>,               // should be a codeblock
        else_clause: Option<Box<Node<'a>>>, // can be anything walkable
    },

    /// Represents the root of one module. Modules can hold others.
    Root {
        name: String,
        stack: ScopeStack<'a>,
        statements: Children<'a>, // holds executable children, so Chains and Assigments
    },

    /// Represents accessing the value of a variable.
    Variable(Path),

    /// Represents a literal declaration
    Literal(Object<'a>),

    /// Represents an array literal (child nodes within brackets)
    ArrayLiteral(Children<'a>),

    /// Represents an `import` statement
    Import(String),

    /// Represents indexing an array, which is read-only (square brackets)
    Index {
        path: Path,
        index1: Box<Node<'a>>,
        index2: Option<Box<Node<'a>>>,
    },

    /// Represents a try-catch block
    Try {
        block_to_try: Box<Node<'a>>,
        catch: Box<Node<'a>>,
    }
}

#[derive(Clone, Debug)]
pub struct Node<'a> {
    pub token: Option<Token>,
    pub ntype: NodeType<'a>,
}

impl<'a> Node<'a> {
    /// Create a new `Node<'a>Type::Root`
    pub fn new_root(name: String) -> Node<'a> {
        Node {
            token: None,
            ntype: NodeType::Root {
                name,
                stack: ScopeStack::new(),
                statements: Vec::new(),
            },
        }
    }

    /// Add a child to a `Node<'a>Type::Root` and panic if the called node isn't a `Root`
    pub fn root_add_child(&mut self, child: Self) {
        let NodeType::Root {
            name,
            stack,
            mut statements,
        } = self.ntype.clone()
        else {
            panic!("tried to call root_add_child on a non-root node!");
        };

        statements.push(child);

        self.ntype = NodeType::Root {
            name: name.clone(),
            stack: stack.clone(),
            statements: statements.clone(),
        };
    }

    /// Check if this node creates a stopper on the stack
    pub fn is_stopper(&self) -> bool {
        matches!(
            self.ntype,
            NodeType::Continue | NodeType::Break | NodeType::Return(_)
        )
    }

    /// Add a `module::Library` to this `Root`'s stack as an `UnimportedModule`.
    pub fn add_library(&mut self, library: module::Library<'a>) {
        let NodeType::Root {
            name,
            mut stack,
            statements,
        } = self.ntype.clone()
        else {
            panic!("add_library() called on a non-root Node!");
        };
        stack.push(Variable::UnimportedModule(library.into()));

        // update scope
        *self = Self {
            token: self.token.clone(),
            ntype: NodeType::Root {
                name,
                stack,
                statements,
            },
        };
    }

    /// Walk self using your own stack (only for `NodeType::Root`)
    pub fn walk_as_mod(&mut self, include_std: bool) -> Result<(), Error> {
        let NodeType::Root {
            name,
            mut stack,
            statements,
        } = self.ntype.clone()
        else {
            panic!("Node::walk_as_mod() called on a non-root node!");
        };

        // check for standard library. if it's not found, include it
        if stack.lookup(&String::from("std")).is_none() && include_std {
            include::include(&mut stack);
        }

        let result = self.walk(&mut stack);

        // reset self
        *self = Self {
            token: self.token.clone(),
            ntype: NodeType::Root {
                name,
                stack,
                statements,
            },
        };

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
