//! # Object
//! This module holds code having to do with Ile objects- functions, methods, and data.
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
//! On the back end, the type is stored as an immutable reference to a `DataType` struct stored in
//! the interpreter. The attributes are stored as a `HashMap`.

pub mod data;
pub use data::DataType;
use std::collections::HashMap;

/// # Object
/// An object is anything that Ile code can see. It can be of three classifications: Data, Function,
/// or Method. See the module's docstring for more explanation.
#[derive(Clone)]
pub enum Object<'a> {
    Function {
        name: &'a str,
    },

    Method {
        name: &'a str,
    },

    Data {
        data_type: &'a DataType<'a>,
        attributes: HashMap<&'a str, Object<'a>>,
    },
}
