//! # interface
//! This module holds logic to wrap Rust functions to be usable within Ile.

use crate::*;

/// # FunctionResult<'a>
/// This is an alias for `Result<Option<Object<'a>>, Error>` and represents the return value of a
/// function. If the function succeeds, the `Option` is its return value. If it fails, the `Error`
/// is used to determine what went wrong.
pub type FunctionResult<'a> = Result<Option<Object<'a>>, Error>;

/// Takes a function and signature. Automatically adds checks to ensure that provided arguments to
/// the function are the same as in the given signature.
pub fn wrap_function<'a>(function: &'a dyn Fn(FunctionSignature<'a>) -> FunctionResult<'a>, signature: FunctionSignature<'a>) -> Object<'a> {
    Object::Function(Executable::Wrapper { signature, func: function })
}
