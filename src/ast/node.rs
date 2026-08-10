//! # node
//! This module holds code to represent a node in the Abstract Syntax Tree. This can be anything
//! that can show up in source code--a code block holding other nodes, a function call holding
//! values to pass, or things like numbers, booleans or string declarations.

use super::scope::ScopeStack;
use crate::{Object, Variable, TokenType};

use std::collections::HashMap;

/// # Path
/// This is an alias for `Vec<String>` and used to represent a path to an object.
type Path = Vec<String>;

/// # Children<'a>
/// This is an alias for `Vec<Node<'a>>`, used to represent a `Node`'s children.
type Children<'a> = Vec<Node<'a>>;

/// # Node
/// A node is any part of an AST.
#[derive(Clone, Debug)]
pub enum Node<'a> {
    /// Represents a function call
    Call {
        arguments: Children<'a>,

        /// This is the full, written path to the function, separated by object; it might look
        /// something like `["object", "attribute", "method"]`.
        path: Path,
    },

    /// Represents a logic operator (addition, subtraction, comparisons, etc)
    Operator(TokenType),

    /// Represents functions
    CodeBlock {
        chains: Children<'a>, // children should be Chain
        signature: Vec<String>,
    },

    /// Represents a statement. It holds a `Vec` of `Node::Call`, `Node::Variable`, or
    /// `Node::Literal`s to allow for method chaining.
    Chain(Children<'a>), // children should be Call or Literal or Variable

    /// Represents assigning a value to a named variable.
    Assignment {
        path: Path,
        value: Box<Self>, // child should be Chain

        /// Represents whether this is a `let` statement (creating a variable) or a reassignment
        create: bool,
    },

    /// Represents a `DataType` definition.
    /// The read `DataType` type isn't used here because attributes can come from functions, which
    /// are only usable at runtime
    DataType {
        name: String,
        methods: HashMap<String, Self>,
        attributes: HashMap<String, Self>,
    },

    /// Represents a `while` loop.
    While {
        condition: Box<Self>, // should be Chain
        block: Box<Self>, // should be a CodeBlock
    },

    /// Represents a `for` loop.
    For {
        condition: Box<Self>, // should be anything executable
        block: Box<Self>, // should be a CodeBlock
    },

    /// Represents an `if` block.
    If {
        condition: Box<Self>, // should be Chain
        block: Box<Self>, // should be a codeblock
        else_clause: Option<Box<Self>>, // can be anything walkable
    },

    /// Represents the root of one module. Modules can hold others.
    Root {
        name: String,
        stack: ScopeStack<'a>,
        imports: Children<'a>,    // children should be Root
        statements: Children<'a>, // holds executable children, so Chains and Assigments
    },

    /// Represents accessing the value of a variable.
    Variable(Path),

    /// Represents a literal declaration
    Literal(Object<'a>),
}

impl Node<'_> {
    /// Determine if the provided String is a known type or classification
    pub fn is_type_or_class(&mut self, name: String) -> bool {
        let Self::Root { stack, .. } = self else {
            panic!("called is_type_or_class on a non-root Node!");
        };

        let lookup = stack.lookup(&name);
        if let Some(Variable::Datatype { .. }) = lookup {
            return true;
        }

        matches!(name.as_str(), "Integer" | "Boolean" | "Float" | "String" | "Function")
    }

    /// Create a new `Node::Root`
    pub fn new_root(name: String) -> Self {
        Self::Root {
            name,
            stack: ScopeStack::new(),
            imports: Vec::new(),
            statements: Vec::new(),
        }
    }

    /// Add a child to a `Node::Root` and panic if the called node isn't a `Root`
    pub fn root_add_child(&mut self, child: Self) {
        let Self::Root { name, stack, imports, statements } = self else {
            panic!("tried to call root_add_child on a non-root node!");
        };

        statements.push(child);

        *self = Self::Root {
            name: name.clone(),
            stack: stack.clone(),
            imports: imports.clone(),
            statements: statements.clone()
        };
    }
}
