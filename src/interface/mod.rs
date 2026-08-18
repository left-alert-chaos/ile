//! # interface
//! This module holds logic to wrap Rust functions to be usable within Ile.

pub mod module;

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

/// Takes a &str and converts it into a FunctionSignature<'_>
/// Uses default values, so the values inside the objects aren't very helpful.
#[macro_export]
macro_rules! signature {
    ( $( $x:expr ),* ) => {
        {
            let mut objects = Vec::new();
            $(
                objects.push(match $x {
                    "int" => Object::Integer(0),
                    "float" => Object::Float(0.0),
                    "bool" => Object::Boolean(false),
                    "string" => Object::String(String::new()),
                    "array" => Object::Array(Vec::new()),
                    "data" => Object::Data(std::collections::HashMap::new()),
                    _ => panic!("Only int, float, bool, string, array, and data are allowed in signature macro"),
                });
            )*
            objects
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_signature() {
        let generated = signature!("string", "int");
        assert_eq!(generated[0].string().unwrap(), &String::new());
        assert_eq!(generated[1].integer().unwrap(), 0);
    }
}
