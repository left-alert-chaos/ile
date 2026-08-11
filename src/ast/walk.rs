//! # walk
//! This module holds the logic to walk (execute) an AST.

use crate::*;

/// # FunctionResult<'a>
/// This is an alias for `Result<Option<Object<'a>>, Error>` and represents the return value of a
/// function. If the function succeeds, the `Option` is its return value. If it fails, the `Error`
/// is used to determine what went wrong.
pub type FunctionResult<'a> = Result<Option<Object<'a>>, Error>;

impl<'a> Node<'a> {
    /// Execute the node.
    pub fn walk(&mut self, stack: &mut scope::ScopeStack) -> FunctionResult {
        match self.ntype {
            NodeType::Assignment { .. } => self.walk_assignment(stack),
            _ => {Ok(None)}
        }
    }

    fn walk_assignment(&self, stack: &mut scope::ScopeStack) -> FunctionResult {
        let NodeType::Assignment { path, value, create } = self.ntype else {
            unreachable!();
        };

        // figure out where the assignment is
        
    }

    fn walk_import(&self, stack: &mut scope::ScopeStack) -> FunctionResult {
        let NodeType::Import(path) = self.ntype else {
            unreachable!();
        };

        let ast = match ast_from_file(path) {
            Ok(ast) => ast,
            Err(reason) => {
                reason.file = 
            }
        }
    }
}
