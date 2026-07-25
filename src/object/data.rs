//! # Data
//! This module is dedicated to data objects and how they are represented. It stores code for
//! `DataType`

use std::collections::HashMap;
use super::Object;

/// # DataType
/// Represents a type an object can have.
/// This holds methods and attributes that must be present in every instance of a type.
#[derive(Clone)]
pub struct DataType<'a> {
    pub methods: HashMap<&'a str, Object<'a>>,
    pub attributes: HashMap<&'a str, DataType<'a>>,
}
