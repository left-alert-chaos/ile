//! # module
//! `module` holds logic to easily create a library module. This is where `Library` is defined.
//! Please see `interface`'s docstring to learn more.

use crate::*;

/// # Library<'a>
/// Represents a library. It is used to build all of its attributes before converting it into an AST
/// node.
///
/// The simplest implementation is:
/// ```rust
/// use ile::*;
/// let node: Node = module::Library::new("my_library_name").into();
/// ```
///
/// Your library will then be imported as "my_library_name".
///
/// It implements `Into<Node<'a>>` and `Into<Variable<'a>>`.
#[derive(Debug, Clone)]
pub struct Library<'a> {
    pub scope: ScopeStack<'a>,
    name: String,
}

impl<'a> Library<'a> {
    /// Create a new `Library`.
    pub fn new(name: impl ToString) -> Self {
        Self {
            scope: ScopeStack::new(),
            name: name.to_string(),
        }
    }

    /// Add a function to a `Library`.
    pub fn add_function(
        &mut self,
        function: &'a dyn Fn(FunctionSignature<'a>) -> FunctionResult<'a>,
        signature: FunctionSignature<'a>,
        name: impl ToString,
    ) {
        let value = wrap_function(function, signature);
        self.scope.push(Variable::Var {
            name: name.to_string(),
            value,
        })
    }

    /// Add another `Library` to this one as a child.
    pub fn add_child(&mut self, child: Self) {
        self.scope.push(Variable::Module(child.into()));
    }
}

impl<'a> From<Library<'a>> for Node<'a> {
    fn from(value: Library<'a>) -> Self {
        Node {
            token: None,
            ntype: NodeType::Root {
                name: value.name,
                stack: value.scope,
                statements: Vec::new(),
            }
        }
    }
}

impl<'a> From<Library<'a>> for Variable<'a> {
    fn from(value: Library<'a>) -> Variable<'a> {
        Self::UnimportedModule(value.into())
    }
}
