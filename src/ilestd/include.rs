//! # include
//! This module holds bindings to Rust functions that are included in the scope when a program
//! starts. This includes `println`.

use crate::*;

pub fn include(scope: &mut ScopeStack<'_>) {
    scope.push(
        Variable::Var {
            name: String::from("println"),
            value: Object::Function(wrap_function(&ile_println, Vec::from([Object::String(String::new())])))
        }
    )
}

fn ile_println(args: FunctionSignature<'_>) -> FunctionResult<'_> {
    if args.len() != 1 {
        return Err(Error::new_rust("println() takes one argument"));
    }

    println!("{:?}", args[0]);
    
    Ok(None)
}
