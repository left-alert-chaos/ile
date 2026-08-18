//! # in_out
//! This is the module holding logic for Ilestd's `io` module.

use crate::*;

use std::fs;

/// Build the `io` module into a `module::Library`.
pub fn build<'a>() -> module::Library<'a> {
    let mut in_out = module::Library::new("io");

    in_out.add_function(&read, signature!("string"), "read");
    in_out.add_function(&write, signature!("string", "string"), "write");

    in_out
}

/// Open a file and return a file descriptor as an `Integer`.
fn read<'a>(s: FunctionSignature<'a>) -> FunctionResult<'a> {
    if s.len() != 1 {
        return Err(Error::new_rust("io.read() takes one argument"));
    }
    let Object::String(fname) = s[0].clone() else {
        return Err(Error::new_rust("io.read() takes a string as an argument"));
    };

    let contents = match fs::read_to_string(fname.clone()) {
        Ok(s) => s,
        Err(_) => return Err(Error::new_rust(format!("couldn't read file '{fname}'"))),
    };

    Ok(
        Some(
            Object::String(contents)
        )
    )
}

/// Open a file and write the given string to it.
fn write<'a>(s: FunctionSignature<'a>) -> FunctionResult<'a> {
    if s.len() != 2 {
        return Err(Error::new_rust("io.write() takes two arguments"));
    }
    let Object::String(fname) = s[0].clone() else {
        return Err(Error::new_rust("io.write()'s first argument is a string"));
    };
    let Object::String(contents) = s[1].clone() else {
        return Err(Error::new_rust("io.write()'s second argument is a string"));
    };

    if std::fs::write(fname.clone(), contents).is_err() {
        return Err(Error::new_rust(format!("couldn't write to '{fname}'")));
    }

    Ok(None)
}
