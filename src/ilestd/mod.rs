//! # ilestd
//! This module holds the source code of Ile's standard library. Some of it is in Rust, and some is
//! in Ile.

pub mod cast;
pub mod in_out;
pub mod include;
pub mod time;
pub mod vebagu;

use crate::*;

use std::collections::HashMap;
use std::fs;

const ITER_SOURCE: &str = include_str!("iter.il");

/// Build the standard library as a `module::Library`.
pub fn build_ile_std<'a>() -> module::Library<'a> {
    let mut ilestd = module::Library::new("std");

    ilestd.add_function(&std_info, Vec::new(), "info");

    // add submodules
    ilestd.add_child(in_out::build());
    ilestd.add_child(vebagu::build());
    ilestd.add_child(time::build());
    ilestd.add_child(cast::build());

    // add ile-written module
    let mut iter = ast_from_str(ITER_SOURCE).unwrap();
    log_iter("Created iter AST");
    iter.walk_as_mod(false).unwrap();
    log_iter("Walked iter AST");
    let NodeType::Root { stack, .. } = iter.ntype else { unreachable!() };
    log_iter("Extracted stack from iter ast");
    let mut iter_library = module::Library::new("iter");
    log_iter("Created iter library");
    iter_library.scope = stack;
    log_iter("Set iter library stack");
    ilestd.add_child(iter_library);
    log_iter("Added iter library to ast children");

    ilestd
}

/// Return some info about the standard library.
/// Doesn't check arguments and uses none.
fn std_info<'a>(_args: FunctionSignature<'a>) -> FunctionResult<'a> {
    let attrs = HashMap::from([(
        "version".to_string(),
        Object::String("0.1.0 dev".to_string()),
    )]);

    Ok(Some(Object::Data(attrs)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raise() {
        let code = r#"
        raise("HAHAHAHAHA");
            "#;
        let mut ast = ast_from_str(code).unwrap();

        let NodeType::Root { mut stack, .. } = ast.ntype.clone() else {
            panic!();
        };
        include::include(&mut stack);

        let res = ast.walk(&mut stack);
        assert_eq!(res.err().unwrap().message, String::from("HAHAHAHAHA"))
    }
}

fn log_iter(info: &str) {
    let bytes = fs::read("iter_info.txt").unwrap();
    let mut contents = String::from_utf8(bytes).unwrap();
    contents.push_str(info);
    contents.push('\n');
    fs::write("iter_info.txt", contents).unwrap();
}
