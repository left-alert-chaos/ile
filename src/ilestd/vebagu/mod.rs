//! # vebagu
//! VeBaGu stands for Very Bad Gui. It is a dumb way of displaying a GUI by sending text to display
//! to a Python script that draws it in a Tkinter window.

use std::{
    os::fd::*,
    io::{Write, PipeWriter, pipe},
    process::Command,
    mem::forget,
    fs,
};

const SCRIPT: &str = include_str!("display.py");

use crate::*;

/// Build VeBaGu as a `module::Library`.
pub fn build<'a>() -> module::Library<'a> {
    let mut vebagu = module::Library::new("vebagu");

    vebagu.add_function(&raw_send, signature!("int", "string"), "raw_send");
    vebagu.add_function(&new, Vec::new(), "new");
    vebagu.add_function(&quit, signature!("int"), "quit");

    vebagu
}

/// Spawn a python subprocess and return the rawfd number of the writer to the stdin
fn new(s: FunctionSignature<'_>) -> FunctionResult<'_> {
    if s.len() > 0 {
        return Err(Error::new_rust("vebagu.new() takes no arguments"));
    }

    // add a temporary file that holds the script
    if fs::write("display.py", SCRIPT).is_err() {
        return Err(Error::new_rust("vebagu.new() failed to create temporary script"));
    }

    // run python3
    let mut script_process = Command::new("python3");
    script_process.arg("display.py");
    let (input_reader, input_writer) = match pipe() {
        Ok(p) => p,
        Err(_) => return Err(Error::new_rust("vebagu.new() failed to create an input pipe")),
    };
    script_process.stdin(input_reader);
    //input_writer.write_all("h1 hello\n".as_bytes());

    // attempt to spawn the process
    if script_process.spawn().is_err() {
        return Err(Error::new_rust("vebagu.new() failed to spawn the python subprocess"));
    }

    // safely dispose of the writer to not close the pipe
    let int_fd = input_writer.as_raw_fd() as i64;
    forget(input_writer);

    Ok(Some(Object::Integer(int_fd)))
}

/// Take a raw fd number and write text to it
fn raw_send(s: FunctionSignature<'_>) -> FunctionResult<'_> {
    if s.len() != 2 {
        return Err(Error::new_rust("vebagu.raw_send() takes two arguments"));
    }
    let Object::Integer(num) = s[0] else {
        return Err(Error::new_rust("vebagu.raw_send() takes an integer and a string as arguments; the integer is missing"));
    };
    let Object::String(mut text) = s[1].clone() else {
        return Err(Error::new_rust("vebagu.raw_send() takes an integer and a string as arguments; the string is missing"));
    };

    text.push('\n');

    let raw_fd = num as RawFd;
    let mut writer = unsafe { PipeWriter::from_raw_fd(raw_fd) };

    if writer.write_all(text.as_bytes()).is_err() {
        forget(writer);
        Err(Error::new_rust("writer.write_all() failed"))
    } else {
        forget(writer);
        Ok(None)
    }

}

/// Take a raw fd number and close the pipe
fn quit(s: FunctionSignature<'_>) -> FunctionResult<'_> {
    if s.len() != 1 {
        return Err(Error::new_rust("vebagu.quit() takes one argument"));
    }
    let Object::Integer(num) = s[0] else {
        return Err(Error::new_rust("vebagu.quit() takes an integer as its only argument"));
    };

    let raw_fd = num as RawFd;
    let mut writer = unsafe { PipeWriter::from_raw_fd(raw_fd) };
    
    if writer.write_all("quit".as_bytes()).is_err() {
        return Err(Error::new_rust("writer.write_all() failed"))
    }

    // remove temp script
    let _ = std::fs::remove_file("display.py");

    Ok(None)
}
