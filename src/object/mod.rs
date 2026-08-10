//! # Object
//! This module holds code having to do with Ile objects- functions, primitives, methods, and data.
//! These are classifications, which are distinct from types. A type is a way of classifying data
//! objects.
//!
//! # Overview
//! It is important to make the distinction between object classifications; data is any object
//! that is not callable and represents some piece of information. Functions are callable objects.
//! Methods are callable objects that can be stored in a data type's method table.
//!
//! # Method tables
//! To avoid having to store a set of large methods on the backend for every instantiated object,
//! data types store a table of methods and their names. When a method is called on a data object,
//! the interpreter searches its type's method table. If it finds a method of the called name, it
//! executes it. If it doesn't, it searches the object's attributes for a method of the same name.
//! If one is found, that is executed. If not, an error is raised.
//!
//! # Data
//! Most things a developer working in Ile will encounter are data. Data is any object that is not
//! callable. It is the only object classification that can have attributes or hold other objects.
//! Every data object is stored as 2 things: its type and its attributes.
//!
//! On the back end, the type is stored as an immutable reference to a struct implementing `DataType`
//! stored in the interpreter. The attributes are stored as a `HashMap`.
//!
//! # Primitive types
//! There are 4 primitive types: `Integer`, `Float`, `Boolean`, and `String`. They are thin wrappers
//! around Rust types and have no methods. To interact with them, you must use external functions.

pub mod data;
pub use data::DataType;
pub mod builtins;
pub use builtins::*;
use std::collections::HashMap;

/// # Object
/// An object is anything that Ile code can see. It can be of four classifications: Data, Function,
/// primitive, or Method. See the module's docstring for more explanation.
#[derive(Clone, Debug)]
pub enum Object<'a> {
    /// Holds a code block or wrapped function
    Function(Executable<'a>),

    /// Holds a dynamic value that has a type, attributes, and methods.
    Data {
        data_type: &'a DataType<'a>,
        attributes: HashMap<&'a str, Object<'a>>,
    },

    /// Primitive number type
    Integer(i64),

    /// Primitive floating-point type
    Float(f64),

    /// Primitive boolean type
    Boolean(bool),

    /// Primitive string type
    String(String),
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
    pub fn data<'b>(&'b self) -> Option<(&'b DataType<'b>, HashMap<&'b str, Object<'b>>)>
    where
        'b: 'a,
    {
        if let Self::Data {
            data_type,
            attributes,
        } = self
        {
            Some((*data_type, attributes.clone()))
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
}

impl<'a> Variable<'a> {
    /// Return whether this `Variable` is a `StackDivider` or a `Var`
    pub fn is_divider(&self) -> bool {
        match self {
            Self::StackDivider(_) => true,
            _ => false,
        }
    }

    /// Return the name of the variable, if it has one.
    pub fn name(&self) -> Option<String> {
        match self {
            Self::Var { name, .. } => Some(name.clone()),
            Self::Datatype { name, .. } => Some(name.clone()),
            Self::StackDivider(_) => None,
        }
    }

    /// Creates a new `Variable::Datatype` from a given `DataType`
    pub fn new_datatype(dt: DataType<'a>) -> Self {
        Self::Datatype {
            name: dt.name.clone(),
            dt: dt,
        }
    }
}

/// # ScopeType
/// This represents any of the kinds of dividers there are. This is used to determine how far back
/// to pop off the stack when a scope ends or a `return` statement is encountered.
#[derive(Clone, Copy, Debug)]
pub enum ScopeType {
    /// Represents a function scope
    Function,

    /// Represents a loop scope
    Loop,
}
