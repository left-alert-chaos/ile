//! # ilestd
//! This module holds the source code of Ile's standard library. Some of it is in Rust, and some is
//! in Ile.

pub mod include;
pub mod vebagu;

use crate::*;

use std::collections::HashMap;

/// Build the standard library as a `module::Library`.
pub fn build_ile_std<'a>() -> module::Library<'a> {
    let mut ilestd = module::Library::new("std");

    ilestd.add_function(&std_info, Vec::new(), "info");

    ilestd
}

/// Return some info about the standard library.
/// Doesn't check arguments and uses none.
fn std_info<'a>(_args: FunctionSignature<'a>) -> FunctionResult<'a> {
    let attrs = HashMap::from([
        ("version".to_string(), Object::String("0.1.0 dev".to_string())),
    ]);

    Ok(
        Some(
            Object::Data(attrs)
        )
    )
}
