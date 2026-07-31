//! # Data
//! This module is dedicated to data objects and how they are represented. It stores code for
//! `DataType`

use std::collections::HashMap;
use super::Object;

/// # DataType
/// This trait is implemented by all objects that represent types.
pub trait DataType<'a> {
    fn methods(&self) -> HashMap<&str, Object<'a>>;
    fn attributes(&self) -> HashMap<&str, Object<'a>>;
    fn name(&self) -> String;
}
