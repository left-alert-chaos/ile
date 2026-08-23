//! # string
//! This module holds the source code for the `string` module of the standard library.

use crate::*;

pub fn build<'a>() -> module::Library<'a> {
    let mut lib = module::Library::new("string");

    lib.add_function(&characters, signature!("string"), "chars");
    lib.add_function(&split, signature!("string", "string"), "split");

    lib
}

fn characters(args: FunctionSignature<'_>) -> FunctionResult<'_> {
    if args.len() != 1 {
        return Err(Error::new_rust(
            "string.characters() only takes one argument",
        ));
    }
    let Object::String(s) = args[0].clone() else {
        return Err(Error::new_rust(
            "string.characters() takes a string as an argument",
        ));
    };

    let mut characters = Vec::new();

    for character in s.chars() {
        characters.push(Object::String(character.to_string()));
    }

    Ok(Some(Object::Array(characters)))
}

fn split(args: FunctionSignature<'_>) -> FunctionResult<'_> {
    // get arguments
    if args.len() < 1 {
        return Err(Error::new_rust(
            "string.split() only takes one or two argument",
        ));
    }
    let Object::String(s) = args[0].clone() else {
        return Err(Error::new_rust(
            "string.split() takes a string as an argument",
        ));
    };
    let mut pattern = " ".to_string();
    if args.len() == 2 {
        let Object::String(split) = args[1].clone() else {
            return Err(Error::new_rust(
                "string.split()'s second argument is a string",
            ));
        };
        pattern = split;
    }

    let mut words = Vec::new();
    let mut buffer = Vec::new();

    // iterate over characters
    for character in s.chars() {
        buffer.push(character);
        let word: String = buffer.iter().collect();

        if word.ends_with(&pattern) {
            for _ in pattern.chars() {
                buffer.pop();
            }
            words.push(Object::String(word));
            buffer.clear();
        }
    }
    words.push(Object::String(buffer.iter().collect()));

    Ok(Some(Object::Array(words)))
}
