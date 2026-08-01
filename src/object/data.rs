//! # Data
//! This module is dedicated to data objects and how they are represented. It stores code for
//! `DataType`

use std::collections::HashMap;
use super::Object;

/// # DataType
/// This struct represents a data type in Ile. It contains the methods and attributes that objects
/// that have a type must have.
pub struct DataType<'a> {
    pub methods: HashMap<&'a str, Object<'a>>,
    pub attributes: HashMap<&'a str, Object<'a>>,
    pub name: String,
}
