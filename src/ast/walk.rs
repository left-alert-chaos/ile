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
    pub fn walk(&mut self, stack: &mut scope::ScopeStack<'a>) -> FunctionResult<'a> {
        match self.ntype {
            NodeType::Assignment { .. } => self.walk_assignment(stack),
            NodeType::Import(_) => self.walk_import(stack),
            _ => {Ok(None)}
        }
    }

    /// Walk an assignment
    fn walk_assignment(&self, stack: &mut scope::ScopeStack<'a>) -> FunctionResult<'a> {
        let NodeType::Assignment { mut path, mut value, create } = self.ntype.clone() else {
            unreachable!();
        };

        let name = match path.pop() {
            Some(name) => name,
            None => return Err(Error::new_runtime(self.token.clone(), "assignment has no name")),
        };

        let walk_res = match value.walk(stack)? {
            Some(res) => res,
            None => return Err(Error::new_runtime(self.token.clone(), "assigned node returned nothing")),
        };

        if path.is_empty() {
            // assign in the current scope
            let var = Variable::Var {
                name: name.clone(),
                value: walk_res,
            };

            if create {
                stack.push(var);
            } else {
                if let Some(entry) = stack.lookup(&name) {
                    *entry = var;
                } else {
                    return Err(Error::new_runtime(self.token.clone(), format!("can't rea-assign to variable '{name}' that doesn't exist")));
                }
            }
        } else {
            // assign as an attribute
            let obj = stack.path_lookup(&path, &self.token.clone().unwrap())?;
            let Object::Data(attrs) = obj else {
                return Err(Error::new_runtime(self.token.clone(), format!("can't assign attribute to non-data object '{}{name}'", debug_path(&path))));
            };
            attrs.insert(name, walk_res);
            *obj = Object::Data(attrs.clone());
        }

        Ok(None)
    }

    fn walk_import(&self, self_stack: &mut scope::ScopeStack<'a>) -> FunctionResult<'a> {
        let NodeType::Import(path) = self.ntype.clone() else {
            unreachable!();
        };

        let mut ast = match ast_from_file(path) {
            Ok(ast) => ast,
            // add file to error
            Err(mut reason) => {
                reason.file = self.token.clone().unwrap().file.unwrap_or(String::from("unknown"));
                return Err(reason);
            }
        };

        // this uses waaaaay too much memory, but it's what I can come up with
        let NodeType::Root { name, stack, statements } = ast.ntype.clone() else {
            unreachable!();
        };
        ast.walk(self_stack)?;
        ast = Node {
            token: None,
            ntype: NodeType::Root {
                stack,
                name,
                statements,
            },
        };

        let var = Variable::Module(ast.clone());
        self_stack.push(var);

        Ok(None)
    }
}

fn debug_path(path: &Vec<String>) -> String {
    let mut res = String::new();

    for i in path {
        res.push_str(i.as_str());
        res.push('.');
    }

    res
}
