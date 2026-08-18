//! # ilestd
//! This module holds the source code of Ile's standard library. Some of it is in Rust, and some is
//! in Ile.

pub mod include;
pub mod in_out;
pub mod vebagu;

use crate::*;

use std::collections::HashMap;

/// Build the standard library as a `module::Library`.
pub fn build_ile_std<'a>() -> module::Library<'a> {
    let mut ilestd = module::Library::new("std");

    ilestd.add_function(&std_info, Vec::new(), "info");

    // add submodules
    ilestd.add_child(in_out::build());
    ilestd.add_child(vebagu::build());

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
