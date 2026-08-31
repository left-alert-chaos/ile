//! # codegen
//! This module holds the logic to convert an `Instruction<'_>` into a `String` that can be
//! interpreted by the VM.
//!
//! This is implemented with `std::fmt::Display`, funnily enough.

use crate::*;
use super::{Instruction, Location};

use std::fmt;

impl fmt::Display for Instruction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "codegen doesn't work yet")
    }
}

impl fmt::Display for Location<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr;
        let text = match self {
            Self::Result => "RES",
            Self::Var(name) => name.as_str(),
            Self::Operation1 => "OP1",
            Self::Operation2 => "OP2",

            // conversion to &str requires a longer-living value
            Self::Literal(object) => {
                repr = format_literal(object.clone());
                repr.as_str()
            }
            Self::GenericRegister(num) => {
                repr = format!("r{num}");
                repr.as_str()
            }
        };
        write!(f, "{text}")
    }
}

/// Represent an object as a literal declaration
fn format_literal(object: Object<'_>) -> String {
    match object {
        // add quotes
        Object::String(mut s) => {
            s = s.replace("\n", "\\n").replace("\t", "\\t");
            format!("\"{s}\"")
        }
        Object::Float(f) => format!("{f}"),
        Object::Integer(i) => format!("{i}"),
        Object::Boolean(b) => format!("{b}"),
        _ => panic!("Object {object} can't be generated as a literal because it isn't a boolean, number, or string"),
    }
}
