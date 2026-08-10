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

    /*
    /// Add a child to a node
    pub fn add_child(&mut self, child: Self) -> Result<(), String> {
        match self {
            Self::Call { .. } => self.call_add_child(child),
            Self::CodeBlock { .. } => self.code_block_add_child(child),
            Self::Chain(_) => self.chain_add_child(child),
            Self::Assignment { .. } => self.assignment_add_child(child),
            Self::DataType(_) => self.data_type_add_child(child),
            Self::Root { .. } => self.root_add_child(child),
            Self::While { .. } => self.while_add_child(child),
            Self::For { .. } => self.for_add_child(child),
            Self::If { .. } => self.if_add_child(child),
            _ => Err(String::from(
                "internal: Node::Variable and Node::Literal cannot have children assigned to them.",
            )),
        }
    }

    /// Add a child to a `Call`
    fn call_add_child(&mut self, child: Self) -> Result<(), String> {
        let Self::Call { arguments, path } = self else {
            panic!(
                "Node::call_add_child(): Tried to add call child but parent isn't a Node::Call!"
            );
        };

        arguments.push(child);

        *self = Self::Call {
            arguments: arguments.clone(),
            path: path.clone(),
        };

        Ok(())
    }

    /// Add a child to a `CodeBlock`
    fn code_block_add_child(&mut self, child: Self) -> Result<(), String> {
        let Self::CodeBlock { chains, signature } = self else {
            panic!(
                "Node::code_block_add_child(): Tried to add a code block child but parent isn't a Node::CodeBlock!"
            );
        };

        match child {
            Self::Chain(_) => chains.push(child),
            _ => return Err(String::from("code blocks can only have chains as children")),
        }

        *self = Self::CodeBlock {
            chains: chains.clone(),
            signature: signature.clone(),
        };

        Ok(())
    }

    /// Add a child to a `Chain`
    fn chain_add_child(&mut self, child: Self) -> Result<(), String> {
        let Self::Chain(children) = self else {
            panic!(
                "Node::chain_add_child(): Tried to add a chain child but parent isn't a Node::Chain!"
            );
        };

        children.push(child);
        *self = Self::Chain(children.clone());
        Ok(())
    }

    /// Set value of `Assignment`
    fn assignment_add_child(&mut self, child: Self) -> Result<(), String> {
        let Self::Assignment { path, create, .. } = self else {
            panic!(
                "Node::assignment_add_child(): Tried to set value but parent isn't Node::Assignment!"
            );
        };

        *self = Self::Assignment {
            path: path.clone(),
            value: Box::new(child), // put the child in a box, haha funny
            create: create.clone(),
        };

        Ok(())
    }

    /// Add an attribute or method to a DataType
    fn data_type_add_child(&mut self, child: Self) -> Result<(), String> {
        let Self::DataType(data_type) = self else {
            panic!(
                "Node::data_type_add_child(): Tried to add a DataType child but parent isn't Node::DataType!"
            );
        };

        let Self::Assignment { path, value, .. } = child else {
            return Err(String::from(
                "only assignments are allowed in datatype definitions",
            ));
        };

        // only code blocks and literals are allowed to be assigned in datatype definitions
        match *value {
            Node::CodeBlock { .. } => {
                // create a Function out of the CodeBlock
                let func = Object::Function(Executable::CodeBlock(value));
                data_type.methods.insert(path, func);
            }
            Node::Literal(obj) => {
                let _ = data_type.attributes.insert(name, obj);
            }
            _ => {
                return Err(String::from(
                    "only assignments to functions or literals are allowed in datatype definitions",
                ));
            }
        }

        *self = Self::DataType(data_type.clone());

        Ok(())
    }

    /// Add a child to a Root, which can either be another Root or a Chain
    fn root_add_child(&mut self, child: Self) -> Result<(), String> {
        let Self::Root {
            imports,
            statements,
            stack,
            ..
        } = self
        else {
            panic!(
                "Node::root_add_child(): Tried to add a Root child but parent isn't Node::Root!"
            );
        };

        // determine how to store child based on child type
        match child {
            Self::Root { .. } => {
                imports.push(child);
            }
            Self::Chain { .. } | Self::Assignment { .. } => {
                statements.push(child);
            }
            Self::DataType(dt) => stack.push(Variable::new_datatype(dt)),
            _ => {
                return Err(String::from(
                    "only data type definitions, chains, assignments and imports are allowed in modules.",
                ));
            }
        }

        Ok(())
    }

    fn while_add_child(&mut self, child: Self) -> Result<(), String> {
        let Self::While { condition, chains } = self else {
            panic!(
                "Node::while_add_child(): Tried to add a While child but parent isn't Node::While!"
            );
        };

        if condition.is_none() {
            *condition = Some(Box::new(child));
        } else {
            chains.push(child);
        }

        *self = Self::While {
            condition: condition.clone(),
            chains: chains.clone(),
        };

        Ok(())
    }

    fn for_add_child(&mut self, child: Self) -> Result<(), String> {
        let Self::For { condition, chains } = self else {
            panic!("Node::for_add_child(): Tried to add a For child but paren't isn't Node::For!");
        };

        if condition.is_none() {
            *condition = Some(Box::new(child));
        } else {
            chains.push(child);
        }

        *self = Self::For {
            condition: condition.clone(),
            chains: chains.clone(),
        };

        Ok(())
    }

    fn if_add_child(&mut self, child: Self) -> Result<(), String> {
        let Self::If {
            condition,
            chains,
            secondary_conditions,
        } = self
        else {
            panic!("Node::if_add_child(): Tried to add an If child but parent isn't Node::If!");
        };

        if condition.is_none() {
            *condition = Some(Box::new(child));
        } else {
            match child {
                Self::Chain { .. } => chains.push(child),
                Self::If { .. } => secondary_conditions.push(child),
                _ => {
                    return Err(String::from(
                        "if blocks can only have conditions, chains, and blocks as children",
                    ));
                }
            }
        }

        *self = Self::If {
            condition: condition.clone(),
            chains: chains.clone(),
            secondary_conditions: secondary_conditions.clone(),
        };

        Ok(())
    }
    */
}
