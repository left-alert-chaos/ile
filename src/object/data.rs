//! # Data
//! This module is dedicated to data objects and how they are represented. It stores code for
//! `DataType`

use super::Object;
use std::collections::HashMap;

/// # DataType
/// This struct represents a data type in Ile. It contains the methods and attributes that objects
/// that have a type must have.
#[derive(Clone, Debug)]
pub struct DataType<'a> {
    pub attributes: HashMap<String, Object<'a>>,
    pub name: String,
}
