//! # Object
//! This module holds code having to do with Ile objects- functions, primitives, and data.
//! These are classifications, which are distinct from types. A type is a way of classifying data
//! objects.
//!
//! # Overview
//! It is important to make the distinction between object classifications; data is any object
//! that is not callable and represents some piece of information. Functions are callable objects.
//!
//! # Data
//! Most things a developer working in Ile will encounter are data. Data is any object that is not
//! callable and not a primitive. It is the only object classification that can have attributes or
//! hold other objects. It is stored as a `HashMap` of attributes.
//!
//! # Primitive types
//! There are 5 primitive types: `Integer`, `Float`, `Boolean`, `Array`, and `String`. They are
//! thin wrappers around Rust types and have no methods. To interact with them, you must use external
//! functions.

pub mod data;
pub use data::DataType;
pub mod builtins;
pub use builtins::*;
use std::collections::HashMap;

use crate::*;

/// # Object
/// An object is anything that Ile code can see. It can be of four classifications: Data, Function,
/// primitive, or Method. See the module's docstring for more explanation.
#[derive(Clone, Debug)]
pub enum Object<'a> {
    /// Holds a code block or wrapped function
    Function(Executable<'a>),

    /// Holds a dynamic value that has a type, attributes, and methods.
    Data(HashMap<String, Object<'a>>),

    /// Primitive number type
    Integer(i64),

    /// Primitive floating-point type
    Float(f64),

    /// Primitive boolean type
    Boolean(bool),

    /// Primitive string type
    String(String),

    /// Primitive array type
    Array(Vec<Self>),
}

impl<'a> Object<'a> {
    /// Determine if the object is a function. If it is, the underlying `Executable<'a>` will be
    /// returned in an `Option`.
    pub fn function(&self) -> Option<Executable<'a>> {
        if let Self::Function(exec) = self {
            Some(exec.clone())
        } else {
            None
        }
    }

    /// Determine if the object is data and return related info if it is.
    pub fn data<'b>(&'b self) -> Option<HashMap<String, Object<'b>>>
    where
        'b: 'a,
    {
        if let Self::Data(attributes) = self {
            Some(attributes.clone())
        } else {
            None
        }
    }

    /// Determine if the object is an Integer and return the underlying `i64` if it is.
    pub fn integer(&self) -> Option<i64> {
        if let Self::Integer(x) = self {
            Some(*x)
        } else {
            None
        }
    }

    /// Determine if the object is a Float and return underlying `f64` if it is
    pub fn float(&self) -> Option<f64> {
        if let Self::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    /// Determine if the object is a Boolean and return underlying `bool` if it is.
    pub fn boolean(&self) -> Option<bool> {
        if let Self::Boolean(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    /// Determine if the object is a String and return a reference to underlying `String` if it is.
    pub fn string(&self) -> Option<&String> {
        if let Self::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// Determine if the object is an Array and return a reference to underlying `Vec<Object>` if it
    /// is.
    pub fn array(&'a self) -> Option<&'a Vec<Object<'a>>> {
        if let Self::Array(a) = self {
            Some(a)
        } else {
            None
        }
    }

    /// Determine if this Object has the same classification as the other object. This doesn't
    /// compare `DataType`s or underlying values, just classifications.
    pub fn has_same_classification_as<'b>(&'b self, other: &'b Self) -> bool
    where
        'b: 'a,
    {
        self.function().is_some() && other.function().is_some()
            || self.integer().is_some() && other.integer().is_some()
            || self.float().is_some() && other.float().is_some()
            || self.data().is_some() && other.data().is_some()
            || self.boolean().is_some() && other.boolean().is_some()
            || self.string().is_some() && other.string().is_some()
            || self.array().is_some() && other.array().is_some()
    }

    /// Attempt to create a primitive `Object` from a `Token`
    pub fn from_token(token: Token) -> Result<Self, String> {
        match token.ttype {
            TokenType::String(s) => Ok(Self::String(s)),
            TokenType::Integer(i) => Ok(Self::Integer(i)),
            TokenType::Float(f) => Ok(Self::Float(f)),
            TokenType::Boolean(b) => Ok(Self::Boolean(b)),
            _ => Err(format!("only String, Integer, Float, and Boolean tokens can become objects, not {:?}", token.ttype)),
        }
    }
}

/// # Variable
/// This is a small struct that wraps `Object`. It adds a name field for named values and represents
/// all values with names, be them attributes or variables.
#[derive(Clone, Debug)]
pub enum Variable<'a> {
    /// This is the default variant and represents all values.
    Var { name: String, value: Object<'a> },

    /// This variant represents a `datatype`, which is stored on the stack like all other values.
    Datatype { name: String, dt: DataType<'a> },

    /// This variant represents a switch between scopes and is used to determine what part of the
    /// stack to keep what what part to remove.
    StackDivider(Option<ScopeType>),

    /// Represents an imported module
    Module(Node<'a>),
    
    /// Represents a return value
    Return(Option<Object<'a>>),
}

impl<'a> Variable<'a> {
    /// Return whether this `Variable` is a `StackDivider` or a `Var`
    pub fn is_divider(&self) -> bool {
        matches!(self, Self::StackDivider(_))
    }

    /// Return the name of the variable, if it has one.
    pub fn name(&self) -> Option<String> {
        match self {
            Self::Var { name, .. } => Some(name.clone()),
            Self::Datatype { name, .. } => Some(name.clone()),
            Self::StackDivider(_) => None,
            Self::Module(Node { ntype: NodeType::Root { name, .. }, .. }) => Some(name.clone()),
            _ => unreachable!(),
        }
    }

    /// Creates a new `Variable::Datatype` from a given `DataType`
    pub fn new_datatype(dt: DataType<'a>) -> Self {
        Self::Datatype {
            name: dt.name.clone(),
            dt,
        }
    }
}

/// # ScopeType
/// This represents any of the kinds of dividers there are. This is used to determine how far back
/// to pop off the stack when a scope ends or a `return` statement is encountered.
#[derive(Clone, Debug)]
pub enum ScopeType {
    /// Represents a function scope
    /// The `Vec` is the path to the called function.
    Function(Vec<String>),

    /// Represents a loop scope
    Loop,
}
