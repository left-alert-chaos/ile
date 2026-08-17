//! # module
//! `module` holds logic to easily create a library module.

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

    /// Add another `Library` to this one as a child.
    pub fn add_child(&mut self, child: Self) {
        self.scope.push(Variable::Module(child.into()));
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

impl<'a> Into<Variable<'a>> for Library<'a> {
    fn into(self) -> Variable<'a> {
        Variable::UnimportedModule(self.into())        
    }
}
