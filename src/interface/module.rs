//! # module
//! `module` holds logic to easily create a library module.

use crate::*;

/// # Library<'a>
/// Represents a library. It is used to build all of its attributes before converting it into an AST
/// node.
///
/// The simplest implementation is:
/// ```rust
/// let node = Library::new("my_library_name").into();
/// ```
///
/// It implements `Into<Node<'a>>`.
pub struct Library<'a> {
    scope: ScopeStack<'a>,
    name: String,
}

impl<'a> Library<'a> {
    /// Create a new `Library`.
    pub fn new(name: impl ToString) -> Self {
        Self {
            scope: ScopeStack::new(),
            name: name.to_string()
        }
    }

    /// Add a function to a `Library`.
    pub fn add_function(&mut self, function: &'a dyn Fn(FunctionSignature<'a>) -> FunctionResult<'a>, signature: FunctionSignature<'a>, name: impl ToString) {
        let value = wrap_function(function, signature);
        self.scope.push(
            Variable::Var {
                name: name.to_string(),
                value,
            }
        )
    }
}

impl<'a> Into<Node<'a>> for Library<'a> {
    fn into(self) -> Node<'a> {
        Node {
            token: None,
            ntype: NodeType::Root {
                name: self.name,
                stack: self.scope,
                statements: Vec::new(),
            }
        }
    }
}
