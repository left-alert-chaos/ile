//! # include
//! This module holds bindings to Rust functions that are included in the scope when a program
//! starts. This includes `println`.

use crate::*;

use std::io::{self, Write};

/// Push all objects that are on the scope automatically onto the scope.
pub fn include(scope: &mut ScopeStack<'_>) {
    scope.push(
        Variable::Var {
            name: String::from("println"),
            value: wrap_function(&ile_println, signature!("string")),
        }
    );
    scope.push(
        Variable::Var {
            name: String::from("input"),
            value: wrap_function(&input, signature!("string")),
        }
    );
    scope.push(
        Variable::Var {
            name: String::from("eprintln"),
            value: wrap_function(&ile_eprintln, signature!("string")),
        }
    );
    scope.push(
        Variable::Var {
            name: String::from("print"),
            value: wrap_function(&ile_print, signature!("string")),
        }
    );
    scope.push(
        Variable::Var {
            name: String::from("eprint"),
            value: wrap_function(&ile_eprint, signature!("string")),
        }
    )
}

fn ile_println(args: FunctionSignature<'_>) -> FunctionResult<'_> {
    if args.len() != 1 {
        return Err(Error::new_rust("println() takes one argument"));
    }

    println!("{}", args[0]);
    
    Ok(None)
}

fn ile_eprintln(args: FunctionSignature<'_>) -> FunctionResult<'_> {
    if args.len() != 1 {
        return Err(Error::new_rust("eprintln() takes one argument"));
    }

    println!("{}", args[0]);
    
    Ok(None)
}

fn ile_print(args: FunctionSignature<'_>) -> FunctionResult<'_> {
    if args.len() != 1 {
        return Err(Error::new_rust("print() takes one argument"));
    }

    print!("{}", args[0]);

    Ok(None)
}

fn ile_eprint(args: FunctionSignature<'_>) -> FunctionResult<'_> {
    if args.len() != 1 {
        return Err(Error::new_rust("eprint() takes one argument"));
    }
    
    eprint!("{}", args[0]);

    Ok(None)
}

// reads a line of `stdin` and returns it as an `Object::String`
fn input(args: FunctionSignature<'_>) -> FunctionResult<'_> {
    if args.len() == 1 {
        print!("{}", args[0]);
        let _ = io::stdout().flush();
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
