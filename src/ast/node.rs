//! # node
//! This module holds code to represent a node in the Abstract Syntax Tree. This can be anything
//! that can show up in source code--a code block holding other nodes, a function call holding
//! values to pass, or things like numbers, booleans or string declarations.

use crate::{
    FunctionSignature,
    Object,
    DataType,
    Executable,
};
use super::scope::ScopeStack;

use std::ops::Deref;

/// # Path
/// This is an alias for `Vec<String>` and used to represent a path to an object.
type Path = Vec<String>;

/// # Children<'a>
/// This is an alias for `Vec<Node<'a>>`, used to represent a `Node`'s children.
type Children<'a> = Vec<Node<'a>>;

/// # Node
/// A node is any part of an AST.
#[derive(Clone)]
pub enum Node<'a> {
    /// Represents a function call
    Call {
        arguments: Children<'a>,

        /// This is the full, written path to the function, separated by object; it might look
        /// something like `["object", "attribute", "method"]`.
        path: Path,
    },
    
    /// Represents functions
    CodeBlock(Children<'a>), // children should be Chain

    /// Represents a statement. It holds a `Vec` of `Node::Call`, `Node::Variable`, or
    /// `Node::Literal`s to allow for method chaining.
    Chain(Children<'a>), // children should be Call or Literal or Variable

    /// Represents assigning a value to a named variable.
    Assignment {
        name: String,
        value: Box<Self>, // child should be Chain
    },

    /// Represents a `DataType` definition.
    DataType(DataType<'a>),
    
    /// Represents the root of one module. Modules can hold others.
    Root {
        name: String,
        stack: ScopeStack<'a>,
        imports: Children<'a>, // children should be Root
        types: Vec<DataType<'a>>,
        statements: Children<'a>, // holds executable children, so Chains and Assigments
    },

    /// Represents accessing the value of a variable.
    Variable(Path),

    /// Represents a literal declaration
    Literal(Object<'a>),
}

impl Node<'_> {
    /// Create a new `Node::Root`
    pub fn new_root(name: String) -> Self {
        Self::Root {
            name,
            stack: ScopeStack::new(),
            imports: Vec::new(),
            types: Vec::new(),
            statements: Vec::new(),
        }
    }

    /// Add a child to a node
    pub fn add_child(&mut self, child: Self) -> Result<(), String> {
        match self {
            Self::Call { .. } => self.call_add_child(child),
            Self::CodeBlock { .. } => self.code_block_add_child(child),
            Self::Chain(_) => self.chain_add_child(child),
            Self::Assignment { .. } => self.assignment_add_child(child),
            Self::DataType(_) => self.data_type_add_child(child),
            Self::Root { .. } => self.root_add_child(child),
            _ => Err(String::from("internal: Node::Variable and Node::Literal cannot have children assigned to them.")),
        }
    }

    /// Add a child to a `Call`
    fn call_add_child(&mut self, child: Self) -> Result<(), String> {
        let Self::Call { arguments, path } = self else {
            panic!("Node::call_add_child(): Tried to add call child but parent isn't a Node::Call!");
        };

        arguments.push(child);

        *self = Self::Call {
            arguments: arguments.clone(),
            path: path.clone()
        };

        Ok(())
    }

    /// Add a child to a `CodeBlock`
    fn code_block_add_child(&mut self, child: Self) -> Result<(), String> {
        let Self::CodeBlock(children) = self else {
            panic!("Node::code_block_add_child(): Tried to add a code block child but parent isn't a Node::CodeBlock!");
        };

        children.push(child);
        *self = Self::CodeBlock(children.clone());

        Ok(())
    }

    /// Add a child to a `Chain`
    fn chain_add_child(&mut self, child: Self) -> Result<(), String> {
        let Self::Chain(children) = self else {
            panic!("Node::chain_add_child(): Tried to add a chain child but parent isn't a Node::Chain!");
        };

        children.push(child);
        *self = Self::Chain(children.clone());
         Ok(())
    }

    /// Set value of `Assignment`
    fn assignment_add_child(&mut self, child: Self) -> Result<(), String> {
        let Self::Assignment { name, .. } = self else {
            panic!("Node::assignment_add_child(): Tried to set value but parent isn't Node::Assignment!");
        };

        *self = Self::Assignment {
            name: name.clone(),
            value: Box::new(child), // put the child in a box, haha
        };
        
        Ok(())
    }

    /// Add an attribute or method to a DataType
    fn data_type_add_child(&mut self, child: Self) -> Result<(), String> {
        let Self::DataType(data_type) = self else {
            panic!("Node::data_type_add_child(): Tried to add a DataType child but parent isn't Node::DataType!");
        };

        let Self::Assignment { name, value } = child else {
            return Err(String::from("only assignments are allowed in datatype definitions"));
        };

        // only code blocks and literals are allowed to be assigned in datatype definitions
        match *value {
            Node::CodeBlock(_) => {
                // create a Function out of the CodeBlock
                let func = Object::Function(Executable::CodeBlock(value));
                data_type.methods.insert(name, func);
            }
            Node::Literal(obj) => {
                let _ = data_type.attributes.insert(name, obj);
            }
            _ => return Err(String::from("only assignments to functions or literals are allowed in datatype definitions"))
        }

        *self = Self::DataType(data_type.clone());

        Ok(())
    }

    /// Add a child to a Root, which can either be another Root or a Chain
    fn root_add_child(&mut self, child: Self) -> Result<(), String> {
        let Self::Root { imports, types, statements, .. } = self else {
            panic!("Node::root_add_child(): Tried to add a Root child but parent isn't Node::Root!");
        };

        // determine how to store child based on child type
        match child {
            Self::Root { .. } => {
                imports.push(child);
            }
            Self::Chain { .. } | Self::Assignment { .. } => {
                statements.push(child);
            }
            Self::DataType(dt) => types.push(dt),
            _ => return Err(String::from("only data type definitions, chains, assignments and imports are allowed in modules.")),
        }

        Ok(())
    }
}
