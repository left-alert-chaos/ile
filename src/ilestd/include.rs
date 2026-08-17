//! # include
//! This module holds bindings to Rust functions that are included in the scope when a program
//! starts. This includes `println`.

use crate::*;

use std::io;

/// Push all objects that are on the scope automatically onto the scope.
pub fn include(scope: &mut ScopeStack<'_>) {
    scope.push(
        Variable::Var {
            name: String::from("println"),
            value: Object::Function(wrap_function(&ile_println, Vec::from([Object::String(String::new())])))
        }
    );
    scope.push(
        Variable::Var {
            name: String::from("input"),
            value: Object::Function(wrap_function(&input, Vec::from([Object::String(String::new())]))),
        }
    );
}

fn ile_println(args: FunctionSignature<'_>) -> FunctionResult<'_> {
    if args.len() != 1 {
        return Err(Error::new_rust("println() takes one argument"));
    }

    println!("{}\n", args[0]);
    
    Ok(None)
}

// reads a line of `stdin` and returns it as an `Object::String`
fn input(args: FunctionSignature<'_>) -> FunctionResult<'_> {
    if args.len() == 1 {
        println!("{}", args[0]);
    } else if args.len() > 1 {
        return Err(Error::new_rust("input() takes one or no arguments"));
    }

    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    buffer = buffer.trim().to_string();

    Ok(
        Some(
            Object::String(buffer)
        )
    )
}
