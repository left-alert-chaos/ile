//! # walk
//! This module holds the logic to walk (execute) an AST.

use crate::*;

use std::collections::HashMap;

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
            NodeType::DataType { .. } => self.walk_datatype(stack),
            NodeType::Variable(mut path) => {
                Ok(
                    Some(
                        stack.path_lookup(&mut path, &self.token.clone().unwrap())?.clone()
                    )
                )
            }
            NodeType::Operator(_, _, _) => self.walk_operator(stack),
            NodeType::If { .. } => self.walk_if(stack),
            NodeType::For { .. } => self.walk_for(stack),
            NodeType::While { .. } => self.walk_while(stack),
            _ => Ok(None)
        }
    }

    fn walk_for(&self, stack: &mut scope::ScopeStack<'a>) -> FunctionResult<'a> {
        let NodeType::For { mut condition, block } = self.ntype.clone() else {
            unreachable!();
        };

        // repeat until condition doesn't return anything
        loop {
            let condition_result = condition.walk(stack)?;
            if condition_result.is_none() { break; }

            block.walk_block(Vec::new(), stack, &Vec::new(), false)?;

            if stack.is_stopper() { break; }
        }

        Ok(None)
    }

    fn walk_while(&self, stack: &mut scope::ScopeStack<'a>) -> FunctionResult<'a> {
        let NodeType::While { mut condition, block } = self.ntype.clone() else {
            unreachable!();
        };

        // repeat until condition returns false
        loop {
            let condition_result = condition.walk(stack)?;

            // check if it's false; any other value passes, including none
            if let Some(result) = condition_result && result.boolean() == Some(false) { break; }

            block.walk_block(Vec::new(), stack, &Vec::new(), false)?;

            if stack.is_stopper() { break; }
        }

        Ok(None)
    }

    fn walk_if(&self, stack: &mut scope::ScopeStack<'a>) -> FunctionResult<'a> {
        let NodeType::If { mut condition, block, else_clause } = self.ntype.clone() else {
            unreachable!();
        };

        let Some(Object::Boolean(condition_value)) = condition.walk(stack)? else {
            return Err(Error::new_runtime(self.token.clone(), "only booleans can be if conditions"));
        };

        // easiest logic in the history of programming languages
        if condition_value {
            block.walk_block(Vec::new(), stack, &Vec::new(), false)?;
        } else {
            if let Some(else_clause) = else_clause {
                else_clause.walk_block(Vec::new(), stack, &Vec::new(), false)?;
            }
        }

        Ok(None)
    }

    fn walk_operator(&self, stack: &mut scope::ScopeStack<'a>) -> FunctionResult<'a> {
        let NodeType::Operator(operator, mut arm1, mut arm2) = self.ntype.clone() else {
            unreachable!();
        };

        let arm1_value = match arm1.walk(stack)? {
            Some(value) => value,
            None => return Err(Error::new_runtime(self.token.clone(), "operator arm 1 didn't return a value")),
        };
        let arm2_value = match arm2.walk(stack)? {
            Some(value) => value,
            None => return Err(Error::new_runtime(self.token.clone(), "operator arm 2 didn't return a value")),
        };

        if !operator.arms_are_correct(&arm1_value, &arm2_value) {
            return Err(Error::new_runtime(self.token.clone(), format!("one or more arms is an incorrect classification for operator {operator};arms are {arm1_value:?} and {arm2_value:?}")));
        }

        // actually do the operation
        // this is long and ugly, but it works, so who cares
        match operator {
            TokenType::Or => Ok(Some(Object::Boolean(arm1_value.boolean().unwrap() || arm2_value.boolean().unwrap()))),
            TokenType::And => Ok(Some(Object::Boolean(arm1_value.boolean().unwrap() && arm2_value.boolean().unwrap()))),
            TokenType::GreaterThan => {
                match arm1_value {
                    Object::Integer(i1) => Ok(Some(Object::Boolean(i1 > arm2_value.integer().unwrap()))),
                    Object::Float(f1) => Ok(Some(Object::Boolean(f1 > arm2_value.float().unwrap()))),
                    Object::String(s1) => Ok(Some(Object::Boolean(s1.len() > arm2_value.string().unwrap().len()))),
                    _ => Err(Error::new_runtime(self.token.clone(), format!("can't determine if {arm1_value:?} is greater than {arm2_value:?}"))),
                }
            }
            TokenType::LessThan => {
                match arm1_value {
                    Object::Integer(i1) => Ok(Some(Object::Boolean(i1 < arm2_value.integer().unwrap()))),
                    Object::Float(f1) => Ok(Some(Object::Boolean(f1 < arm2_value.float().unwrap()))),
                    Object::String(s1) => Ok(Some(Object::Boolean(s1.len() < arm2_value.string().unwrap().len()))),
                    _ => Err(Error::new_runtime(self.token.clone(), format!("can't determine if {arm1_value:?} is less than {arm2_value:?}"))),
                }
            }
            TokenType::GreaterThanOrEqualTo => {
                match arm1_value {
                    Object::Integer(i1) => Ok(Some(Object::Boolean(i1 >= arm2_value.integer().unwrap()))),
                    Object::Float(f1) => Ok(Some(Object::Boolean(f1 >= arm2_value.float().unwrap()))),
                    Object::String(s1) => Ok(Some(Object::Boolean(s1.len() >= arm2_value.string().unwrap().len()))),
                    _ => Err(Error::new_runtime(self.token.clone(), format!("can't determine if {arm1_value:?} is greater than or equal to {arm2_value:?}"))),
                }
            }
            TokenType::LessThanOrEqualTo => {
                match arm1_value {
                    Object::Integer(i1) => Ok(Some(Object::Boolean(i1 <= arm2_value.integer().unwrap()))),
                    Object::Float(f1) => Ok(Some(Object::Boolean(f1 <= arm2_value.float().unwrap()))),
                    Object::String(s1) => Ok(Some(Object::Boolean(s1.len() <= arm2_value.string().unwrap().len()))),
                    _ => Err(Error::new_runtime(self.token.clone(), format!("can't determine if {arm1_value:?} is less than or equal to {arm2_value:?}"))),
                }
            }
            TokenType::Addition => {
                match arm1_value {
                    Object::Integer(i1) => Ok(Some(Object::Integer(i1 + arm2_value.integer().unwrap()))),
                    Object::Float(f1) => Ok(Some(Object::Float(f1 + arm2_value.float().unwrap()))),
                    Object::String(s1) => Ok(Some(Object::String(s1 + arm2_value.string().unwrap()))),
                    _ => Err(Error::new_runtime(self.token.clone(), format!("can't add {arm1_value:?} to {arm2_value:?}"))),
                }
            }
            TokenType::Subtraction => {
                match arm1_value {
                    Object::Integer(i1) => Ok(Some(Object::Integer(i1 - arm2_value.integer().unwrap()))),
                    Object::Float(f1) => Ok(Some(Object::Float(f1 - arm2_value.float().unwrap()))),
                    _ => Err(Error::new_runtime(self.token.clone(), format!("can't subtract {arm1_value:?} from {arm2_value:?}"))),
                }
            }
            TokenType::Multiplication => {
                match arm1_value {
                    Object::Integer(i1) => Ok(Some(Object::Integer(i1 * arm2_value.integer().unwrap()))),
                    Object::Float(f1) => Ok(Some(Object::Float(f1 * arm2_value.float().unwrap()))),
                    _ => Err(Error::new_runtime(self.token.clone(), format!("can't multiply {arm1_value:?} by {arm2_value:?}"))),
                }
            }
            TokenType::Division => {
                match arm1_value {
                    Object::Integer(i1) => Ok(Some(Object::Float((i1 / arm2_value.integer().unwrap()) as f64))),
                    Object::Float(f1) => Ok(Some(Object::Float(f1 / arm2_value.float().unwrap()))),
                    _ => Err(Error::new_runtime(self.token.clone(), format!("can't divide {arm1_value:?} by {arm2_value:?}"))),
                }
            }
            _ => unreachable!(),
        }
    }

    fn walk_datatype(&self, stack: &mut scope::ScopeStack<'a>) -> FunctionResult<'a> {
        let NodeType::DataType { name, mut attributes } = self.ntype.clone() else {
            unreachable!();
        };

        let mut attribute_values = HashMap::new();

        // evaluate attribute nodes
        for (attr_name, attr_value) in attributes.iter_mut() {
            let node_value = match attr_value.walk(stack)? {
                Some(value) => value,
                None => return Err(Error::new_runtime(self.token.clone(), format!("attribute '{attr_name}' of datatype '{name}' has no value"))),
            };
            attribute_values.insert(attr_name.clone(), node_value);
        }

        // create a stack entry of the datatype
        stack.push(
            Variable::Datatype {
                name: name.clone(),
                dt: DataType {
                    name,
                    attributes: attribute_values,
                }
            }
        );

        Ok(None)
    }

    fn walk_chain(&self, stack: &mut scope::ScopeStack<'a>) -> FunctionResult<'a> {
        // value carried between chained methods
        let mut carried_value = None;

        let NodeType::Chain(calls) = self.ntype.clone() else {
            unreachable!();
        };

        for (index, call) in calls.iter().enumerate() {
            let mut call = call.clone();
            let Some(value) = carried_value.clone() else {
                carried_value = call.walk(stack)?;
                continue;
            };
            let Object::Data(mut attrs) = value else {
                return Err(Error::new_runtime(self.token.clone(), format!("object {value:?} isn't data, so it can't be chained")));
            };
            
            // determine what to do with call or variable
            match call.ntype {
                NodeType::Call { arguments, path } => {
                    let name = path[0].clone();
                    let Some(function) = attrs.get_mut(&name) else {
                        return Err(Error::new_runtime(call.token.clone(), format!("attribute '{name}' doesn't exist, so it can't be called")));
                    };
                    let Object::Function(executable) = function else {
                        return Err(Error::new_runtime(call.token.clone(), format!("attribute '{name}' isn't a function, so it can't be called")));
                    };

                    // determine how to call the function
                    match executable {
                        // re-write the call logic
                        Executable::CodeBlock(node) => {
                            carried_value = node.walk_block(walk_arguments(arguments, stack)?, stack, &Vec::new(), true)?;

                            // don't error if it's the last segment and it wasn't trying to return
                            // anything
                            if carried_value.is_none() && index != calls.len() - 1 {
                                return Err(Error::new_runtime(call.token.clone(), format!("function '{name}' didn't return anything, so can't be chained")))
                            }
                        }
                        Executable::Wrapper { .. } => todo!(),
                    }
                }
                NodeType::Variable(path) => {
                    if let Some(value) = attrs.get(&path[0]) {
                        carried_value = Some(value.clone());
                    } else {
                        return Err(Error::new_runtime(call.token.clone(), format!("chain's carried value has no attribute '{}'", path[0])));
                    }
                }
                _ => return Err(Error::new_runtime(call.token.clone(), "only calls and variable lookups are allowed to be chained")),
            }
        }

        Ok(carried_value)
    }

    /// Walk an assignment
    fn walk_assignment(&self, stack: &mut scope::ScopeStack<'a>) -> FunctionResult<'a> {
        let NodeType::Assignment { mut path, mut value, create } = self.ntype.clone() else {
            unreachable!();
        };

        let Some(name) = path.pop() else {
            return Err(Error::new_runtime(self.token.clone(), "can't assign to empty path"));
        };

        let walk_res = match value.walk(stack)? {
            Some(res) => res,
            None => return Err(Error::new_runtime(self.token.clone(), "assigned node returned nothing")),
        };

        if path.is_empty() {
            path.push(name.clone());
            if create {
                stack.set_path(&path, walk_res.clone(), &self.token.clone().unwrap())?;
            } else {
                if let Ok(variable) = stack.path_lookup(&mut path, &self.token.clone().unwrap()) {
                    *variable = walk_res.clone();
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
            attrs.insert(name, walk_res.clone());
            *obj = Object::Data(attrs.clone());
        }

        Ok(Some(walk_res))
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
        let NodeType::Call { arguments, path } = self.ntype.clone() else {
            unreachable!();
        };

        let arg_objects = walk_arguments(arguments, stack)?;

        // check if there's an available datatype
        if let Ok(dt) = stack.datatype_path_lookup(&mut path.clone(), &self.token.clone().unwrap()) {
            return Ok(
                Some(
                    Object::Data(dt.attributes)
                )
            );
        }

        // get function
        let func = stack.path_lookup(&mut path.clone(), &self.token.clone().unwrap())?;
        let Object::Function(executable) = func.clone() else {
            return Err(Error::new_runtime(self.token.clone(), format!("object '{}' isn't a Function", debug_path(&path))));
        };

        match executable {
            Executable::CodeBlock(block) => {
                block.walk_block(arg_objects, stack, &path, true)
            }
            Executable::Wrapper { signature, func } => {
                func(signature)
            }
        }
    }

    fn walk_block(&self, mut args: Vec<Object<'a>>, stack: &mut scope::ScopeStack<'a>, path: &Vec<String>, is_function: bool) -> FunctionResult<'a> {
        let path = path.clone();
        let NodeType::CodeBlock { chains, signature } = self.ntype.clone() else {
            unreachable!();
        };

        // find the path to the method's parent
        let mut self_path = path.clone();
        self_path.pop();
        let mut set_self = false;

        let args_len = args.len();
        let signature_len = signature.len();

        // ensure that there are the right number of arguments
        if args_len != signature_len {
            let message = if args_len < signature_len {
                // set automatic self value
                if signature[0].as_str() == "self" && signature_len - args_len == 1 {
                    // check that this is, in fact, a method
                    if path.len() > 0 && let Ok(self_object) = stack.path_lookup(&mut self_path.clone(), &self.token.clone().unwrap()) {
                        set_self = true;
                        args.insert(
                            0,
                            self_object.clone(),
                        );
                        String::new()
                    } else {
                        format!("missing one argument; the first argument is 'self', which is confusing because this isn't a method")
                    }
                } else {
                    format!("missing {} arguments", signature_len - args_len)
                }
            } else {
                format!("{} too many arguments", args_len - signature_len)
            };
            if !message.is_empty() {
                return Err(Error::new_runtime(self.token.clone(), message));
            }
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

        // Option<Option<Object>>
        // The first option is whether to return; second is what, if any, to return
        let mut return_value = None;

        for mut statement in chains {
            statement.walk(stack)?;

            if let Some(value) = stack.is_return() {
                return_value = Some(value);
                break;
            }

            if !is_function && stack.is_stopper() {
                break;
            }
        }

        // if it's a method, change the self path to reflect any changes
        if set_self {
            let self_value = stack.lookup(&String::from("self")).unwrap();
            let Variable::Var { value, .. } = self_value.clone() else {
                return Err(Error::new_runtime(self.token.clone(), format!("method '{}' somehow returned with self as a non-variable stack entry; this is confusing and frustrating and shouldn't even be possible", debug_path(&self_path))));
            };
            stack.set_path(
                &mut self_path,
                value,
                &self.token.clone().unwrap(),
            )?;
        }

        // Remove variables from function's scope
        stack.return_cleanup().unwrap();

        // keep the return statement
        if !is_function && return_value.is_some() {
            stack.push(Variable::Return(return_value.clone().unwrap()));
        } else {
        }

        Ok(
            match return_value {
                None => None,
                Some(value) => value,
            }
        )
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
