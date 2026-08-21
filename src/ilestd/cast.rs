//! # cast
//! This module holds the logic for the `cast` module of the standard library, which provides
//! functions to change from one object classification to another.

use crate::*;

pub fn build<'a>() -> module::Library<'a> {
    let mut cast = module::Library::new("cast");

    cast.add_function(&integer, signature!("float"), "integer");
    cast.add_function(&float, signature!("int"), "float");

    cast
}

fn integer(args: FunctionSignature<'_>) -> FunctionResult<'_> {
    if args.len() != 1 {
        return Err(Error::new_rust("cast.integer() takes one argument"));
    }

    match args[0] {
        Object::Data(_) => Err(Error::new_rust("data cannot be cast to an integer")),
        Object::Integer(i) => Ok(Some(Object::Integer(i))),
        Object::Float(f) => Ok(Some(Object::Integer(f as i64))),
        Object::Array(_) => Err(Error::new_rust("arrays cannot be cast to integers")),
        Object::String(_) => Err(Error::new_rust("strings cannot be cast to integers")),
        Object::Boolean(b) => Ok(Some(Object::Integer(b as i64))),
        Object::Function(_) => Err(Error::new_rust("functions cannot be cast to integers")),
    }
}

fn float(args: FunctionSignature<'_>) -> FunctionResult<'_> {
    if args.len() != 1 {
        return Err(Error::new_rust("cast.float() takes one argument"));
    }

    match args[0] {
        Object::Data(_) => Err(Error::new_rust("data cannot be cast to a float")),
        Object::Integer(i) => Ok(Some(Object::Float(i as f64))),
        Object::Float(f) => Ok(Some(Object::Float(f))),
        Object::Array(_) => Err(Error::new_rust("arrays cannot be cast to floats")),
        Object::String(_) => Err(Error::new_rust("strings cannot be cast to floats")),
        Object::Boolean(b) => Ok(Some(Object::Float(b as i64 as f64))),
        Object::Function(_) => Err(Error::new_rust("functions cannot be cast to floats")),
    }
}
