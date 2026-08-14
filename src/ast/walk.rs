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
        match self.ntype.clone() {
            NodeType::Assignment { .. } => self.walk_assignment(stack),
            NodeType::Import(_) => self.walk_import(stack),
            NodeType::Root { statements, .. } => {
                for mut statement in statements {
                    statement.walk(stack)?;
                }
                Ok(None)
            }
            NodeType::Literal(value) => Ok(Some(value)),
            NodeType::ArrayLiteral(_) => self.walk_array(stack),
            NodeType::CodeBlock { .. } => Ok(Some(Object::Function(Executable::CodeBlock(Box::new(self.clone()))))),
            NodeType::Call { .. } => self.walk_call(stack),
            NodeType::Return(mut value) => {
                let value = value.walk(stack)?;
                stack.push(Variable::Return(value));
                Ok(None)
            }
            NodeType::Chain(_) => self.walk_chain(stack),
            _ => Ok(None)
        }
    }

    fn walk_chain(&self, stack: &mut scope::ScopeStack<'a>) -> FunctionResult<'a> {
        // value carried between chained methods
        let mut carried_value = None;

        let NodeType::Chain(calls) = self.ntype.clone() else {
            unreachable!();
        };

        // FIXME: This should scan carried_value's attributes for the method name
        for mut call in calls {
            carried_value = call.walk(stack)?;
        }

        Ok(carried_value)
    }

    /// Walk an assignment
    fn walk_assignment(&self, stack: &mut scope::ScopeStack<'a>) -> FunctionResult<'a> {
        let NodeType::Assignment { mut path, mut value, create } = self.ntype.clone() else {
            unreachable!();
        };

        let name = path[0].clone();

        let walk_res = match value.walk(stack)? {
            Some(res) => res,
            None => return Err(Error::new_runtime(self.token.clone(), "assigned node returned nothing")),
        };

        if path.is_empty() {
            if create {
                stack.set_path(&path, walk_res, &self.token.clone().unwrap())?;
            } else {
                if let Ok(variable) = stack.path_lookup(&mut path, &self.token.clone().unwrap()) {
                    *variable = walk_res;
                } else {
                    return Err(Error::new_runtime(self.token.clone(), format!("can't re-assign to variable '{name}' that doesn't exist")));
                }
            }
        } else {
            // assign as an attribute
            let obj = stack.path_lookup(&mut path.clone(), &self.token.clone().unwrap())?;
            let Object::Data(attrs) = obj else {
                return Err(Error::new_runtime(self.token.clone(), format!("can't assign attribute to non-data object '{}.{name}'", debug_path(&path))));
            };
            attrs.insert(name, walk_res);
            *obj = Object::Data(attrs.clone());
        }

        Ok(None)
    }

    fn walk_import(&self, self_stack: &mut scope::ScopeStack<'a>) -> FunctionResult<'a> {
        println!("Walking import");
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
        let NodeType::Root { name, mut stack, statements } = ast.ntype.clone() else {
            unreachable!();
        };
        ast.walk(&mut stack)?;
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

    fn walk_array(&mut self, stack: &mut scope::ScopeStack<'a>) -> FunctionResult<'a> {
        let NodeType::ArrayLiteral(children) = self.ntype.clone() else {
            unreachable!();
        };

        let mut results = Vec::new();

        for mut child in children {
            match child.walk(stack)? {
                Some(res) => results.push(res),
                None => return Err(Error::new_runtime(child.token.clone(), "array child doesn't return anything")),
            }
        }

        Ok(Some(Object::Array(results)))
    }

    fn walk_call(&mut self, stack: &mut scope::ScopeStack<'a>) -> FunctionResult<'a> {
        println!("Walking call");
        let NodeType::Call { mut arguments, path } = self.ntype.clone() else {
            unreachable!();
        };

        // collect objects from node arguments
        let mut arg_objects = Vec::new();
        for (index, arg) in arguments.iter_mut().enumerate() {
            match arg.walk(stack)? {
                Some(obj) => arg_objects.push(obj),
                None => return Err(Error::new_runtime(arg.token.clone(), format!("argument {index} didn't return anything"))),
            }
        }

        // get function
        let func = stack.path_lookup(&mut path.clone(), &self.token.clone().unwrap())?;
        let Object::Function(executable) = func.clone() else {
            return Err(Error::new_runtime(self.token.clone(), format!("object '{}' isn't a Function", debug_path(&path))));
        };

        match executable {
            Executable::CodeBlock(block) => {
                block.walk_block(arg_objects, stack, &path)
            }
            Executable::Wrapper { signature, func } => {
                func(signature)
            }
        }
    }

    fn walk_block(&self, args: Vec<Object<'a>>, stack: &mut scope::ScopeStack<'a>, path: &Vec<String>) -> FunctionResult<'a> {
        println!("Walking block");
        let NodeType::CodeBlock { chains, signature } = self.ntype.clone() else {
            unreachable!();
        };

        let args_len = args.len();
        let signature_len = signature.len();

        // ensure that there are the right number of arguments
        if args_len != signature_len {
            let message = if args_len < signature_len {
                format!("missing {} arguments", signature_len - args_len)
            } else {
                format!("{} too many arguments", args_len - signature_len)
            };
            return Err(Error::new_runtime(self.token.clone(), message));
        }

        // add a StackDivider
        stack.push(
            Variable::StackDivider( 
                Some(
                    ScopeType::Function(
                        path.clone()
                    )
                )
            )
        );

        // clone arguments onto stack
        for (index, arg) in args.iter().enumerate() {
            let name = signature[index].clone();
            stack.push(
                Variable::Var {
                    name,
                    value: arg.clone(),
                }
            );
        }

        let mut return_value = None;

        for mut statement in chains {
            statement.walk(stack)?;

            if let Some(value) = stack.is_return() {
                println!("Returning {value:?}");
                return_value = value;
                break;
            }
        }

        // Remove variables from function's scope
        stack.return_cleanup().unwrap();

        Ok(return_value)
    }
}

fn debug_path(path: &Vec<String>) -> String {
    let mut res = String::new();

    for i in path {
        res.push_str(i.as_str());
        res.push('.');
    }

    res.pop();

    res
}
