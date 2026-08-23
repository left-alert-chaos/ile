//! # ilestd
//! This module holds the source code of Ile's standard library. Some of it is in Rust, and some is
//! in Ile.
//!
//! If you're including Ile inside your program, you shouldn't need to worry much about the STD, but
//! it _is_ a great way to learn how to write Ile libraries.

pub mod cast;
pub mod in_out;
pub mod include;
pub mod string;
pub mod time;
pub mod vebagu;

use crate::*;

use std::collections::HashMap;

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
    ilestd.add_child(string::build());

    // add ile-written modules
    ilestd.add_child(build_ile_module(ITER_SOURCE, "iter"));

    ilestd
}

fn build_ile_module<'a>(source: &'a str, name: &'a str) -> module::Library<'a> {
    let mut node = ast_from_str(source).unwrap();
    node.walk_as_mod(false).unwrap();
    let NodeType::Root { stack, .. } = node.ntype else {
        unreachable!();
    };
    let mut library = module::Library::new(name);
    library.scope = stack;
    library
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

    #[test]
    fn len() {
        let code = r#"
        import "std";
        let arr = [1, 2, 3];
        let x = std.iter.len(arr);
        "#;
        let mut ast = ast_from_str(code).unwrap();
        ast.walk_as_mod(true).unwrap();
        let NodeType::Root { mut stack, .. } = ast.ntype.clone() else {
            unreachable!();
        };
        let Variable::Var {
            value: Object::Integer(num),
            ..
        } = stack.lookup(&String::from("x")).unwrap()
        else {
            panic!("Couldn't get x");
        };
        assert_eq!(num.clone(), 3);
    }
}
