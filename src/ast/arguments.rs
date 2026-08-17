//! # arguments
//! This module holds the code to convert a `Vec<Node>` into a `Vec<Object>`.

use crate::*;

pub fn walk_arguments<'a>(mut arguments: Vec<Node<'a>>, stack: &mut ScopeStack<'a>) -> Result<Vec<Object<'a>>, Error> {
    let mut objects = Vec::new();

    for (index, arg) in arguments.iter_mut().enumerate() {
        match arg.walk(stack)? {
            Some(obj) => objects.push(obj),
            None => return Err(Error::new_runtime(arg.token.clone(), format!("argument {} didn't return anything", index + 1))),
        }
    }

    Ok(objects)
}
