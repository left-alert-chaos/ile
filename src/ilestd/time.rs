//! # time
//! This module holds code for the `time` module in `std`.

use crate::*;

use std::{
    time::Duration,
    thread,
};

pub fn build<'a>() -> module::Library<'a> {
    let mut time = module::Library::new("time");

    time.add_function(&sleep, signature!("float"), "sleep");

    time
}

fn sleep(args: FunctionSignature<'_>) -> FunctionResult<'_> {
    if args.len() != 1 {
        return Err(Error::new_rust("time.sleep() takes one argument that is either an integer or a float"));
    }

    let seconds;
    let mut ns = 0;
    if let Object::Float(f) = args[0].clone() {
        if f < 0.0 {
            return Err(Error::new_rust("time.sleep() cannot sleep for less than 0 seconds"));
        }

        seconds = f.floor() as u64;
        ns = (f - (seconds as f64)) as u32 * 1000000000;
    } else if let Object::Integer(i) = args[0].clone() {
        if i < 0 {
            return Err(Error::new_rust("time.sleep() cannot sleep for less than 0 seconds"));
        }

        seconds = i as u64;
    } else {
        return Err(Error::new_rust("time.sleep() takes one argument that is either an integer or a float"));
    }

    thread::sleep(Duration::new(seconds, ns));

    Ok(None)
}
