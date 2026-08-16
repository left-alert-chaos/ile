//! # scope
//! The module is for managing various scopes. A _scope_ is a set of variables used by one area of a
//! program. When you jump to another area that uses different variables, you need to be able to
//! swap out your scopes dynamically, which this module provides.

use crate::*;
use crate::error::Error as IleError;

use std::{error::Error, fmt};

/// # ScopeStack
/// This type is used to manage multiple scopes and switch to the currently needed one. Its main
/// mechanism is the `current_stack` field, which is a `Vec` of `Variable`s. When the scope is
/// switched, it counts the `Variable::StackDivider`s until the target scope is reached. It stores
/// the popped values in other fields to enable a switch back to the previous context.
#[derive(Clone, Debug)]
pub struct ScopeStack<'a> {
    current_stack: Vec<Variable<'a>>,
    cached_scopes: Vec<Vec<Variable<'a>>>,
}

impl Default for ScopeStack<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ScopeStack<'a> {
    /// Create a new `ScopeStack` with one `Variable::StackDivider` in the stack already.
    pub fn new() -> Self {
        Self {
            current_stack: Vec::from([Variable::StackDivider(None)]),
            cached_scopes: Vec::new(),
        }
    }

    /// Search for the given variable name in the stack. If it is found, change its value.
    /// Otherwise, create a new variable with the given name and value in the topmost
    /// (shortest-living and most recent) scope.
    /// WARNING: Because the stack is also responsible for holding the `datatype`s, this can
    /// and will overwrite a `datatype` if it is given the right name.
    pub fn set(&mut self, vname: String, value: Object<'a>) {
        for var in self.current_stack.iter_mut() {
            let Some(name) = var.name() else {
                continue;
            };

            if *name == vname {
                *var = Variable::Var { name: vname, value };

                return;
            }
        }

        // if we haven't already returned, create a new var
        self.push(Variable::Var { name: vname, value });
    }

    /// Search for the object at a given path. Similar to `set()`, but used for objects that aren't
    /// immediate children.
    pub fn set_path(&mut self, path: &Vec<String>, value: Object<'a>, token: &Token) -> Result<(), IleError> {
        let mut path = path.clone();
        if path.len() == 1 {
            self.set(path[0].clone(), value);
            return Ok(());
        } else if path.is_empty() {
            // I don't think this is possible, but better safe than sorry.
            return Err(IleError::new_runtime(Some(token.clone()), "can't assign to empty path"));
        }
        
        // remove last segment and search for parent
        let child_name = path.pop().unwrap();
        let parent = self.path_lookup(&mut path, token)?;
        let Object::Data(mut attrs) = parent.clone() else {
            return Err(IleError::new_runtime(Some(token.clone()), "non-data objects can't have attributes assigned"));
        };
        attrs.insert(child_name, value);
        *parent = Object::Data(attrs);
        Ok(())
    }

    /// Push a variable onto the end of the current stack.
    pub fn push(&mut self, var: Variable<'a>) {
        self.current_stack.push(var);
    }

    // I'm so glad this works without any lifetime issues and that it won't break and that it's not
    // going to break and that I'm not going to spend the next week of my life debugging this.
    // That's not going to happen, right? RIGHT!???!
    /// Search in reversed order through all variables on the stack. If one of them has the
    /// requested name, return that one. If none of them do, return None.
    pub fn lookup(&mut self, searched_name: &String) -> Option<&mut Variable<'a>> {
        for var in self.current_stack.iter_mut().rev() {
            if let Some(name) = var.name()
                && name == *searched_name
            {
                return Some(var);
            }
        }

        None
    }

    /// Similar to `lookup()`, but follow a path. It returns a mutable reference to an object that
    /// it got by reading attributes of objects in the path.
    /// For use in the AST walker.
    pub fn path_lookup(&mut self, path: &mut Vec<String>, token: &Token) -> Result<&mut Object<'a>, IleError> {
        if path.is_empty() {
            return Err(IleError::new_runtime(Some(token.clone()), "empty path for lookup"));
        }

        // get the top-level object that everything else is an attribute of
        let first = match self.lookup(&path[0]) {
            Some(Variable::Var { value, .. }) => value,
        
            // If the looked up value is a node, recursively search its stack until you reach an
            // object
            Some(Variable::Module(node)) => {
                let NodeType::Root { stack, .. } = &mut node.ntype else {
                    unreachable!();
                };
                if path.len() > 1 {
                    path.remove(0);
                    stack.path_lookup(path, token)?
                } else {
                    return Err(IleError::new_runtime(Some(token.clone()), "modules aren't objects"));
                }
            }

            None => return Err(IleError::new_runtime(Some(token.clone()), format!("object '{}' doesn't exist", &path[0]))),
            _ => return Err(IleError::new_runtime(Some(token.clone()), "only variables can be looked up with paths")),
        };
        if path.len() == 1 {
            return Ok(first);
        }

        let mut target = first;
        let mut target_name = &path[0];
        for segment in &path[1..] {
            // check if target is data
            let Object::Data(attributes) = target else {
                return Err(IleError::new_runtime(Some(token.clone()), format!("object '{target_name}' isn't Data")));
            };

            target_name = segment;
            match attributes.get_mut(segment.as_str()) {
                Some(t) => target = t,
                None => return Err(IleError::new_runtime(Some(token.clone()), format!("object '{target_name}' doesn't exist"))),
            }
        }

        Ok(target)
    }

    /// Very similar to `path_lookup()`, but searches for a datatype declaration instead.
    pub fn datatype_path_lookup(&mut self, path: &mut Vec<String>, token: &Token) -> Result<DataType<'a>, IleError> {
        if path.is_empty() {
            return Err(IleError::new_runtime(Some(token.clone()), "empty path for lookup"));
        }

        let name = path.pop().unwrap();

        match self.lookup(&name) {
            Some(Variable::Datatype { dt, .. }) => {
                Ok(dt.clone())
            }
            Some(Variable::Module(node)) => {
                let NodeType::Root { stack, .. } = &mut node.ntype else {
                    unreachable!();
                };
                if path.len() > 1 {
                    path.remove(0);
                    stack.datatype_path_lookup(path, token)
                } else {
                    return Err(IleError::new_runtime(Some(token.clone()), "modules aren't usable as datatypes"));
                }
            }
            Some(_) => Err(IleError::new_runtime(Some(token.clone()), format!("path segment '{name}' is an object, not a datatype"))),
            None => Err(IleError::new_runtime(Some(token.clone()), format!("path segment '{name}' doesn't exist")))
        }
    }

    /// In reversed order, count through scopes and remove the specified number. These scopes,
    /// including their `StackDivider`s, are cached and put first in the queue to do-cache.
    pub fn cache_scopes(&mut self, target: u64) -> Result<(), ScopeError> {
        let mut cache = Vec::new();
        let mut scopes: u64 = 0;

        // pop a variable, check if it's a StackDivider, and push onto the cache
        while scopes < target {
            // If the stack is emptied before the target, restore the cached vars and return Err(())
            let Some(var) = self.current_stack.pop() else {
                self.restore_scope(cache);
                return Err(ScopeError::OutOfVars);
            };

            if var.is_divider() {
                scopes += 1;
            }

            cache.push(var);
        }

        self.cached_scopes.push(cache);
        Ok(())
    }

    /// Move the scope that was last cached back onto the stack. This "scope" may be a list of
    /// multiple scopes if the last successful `cache_scopes()` call was passed a target greater than
    /// 1.
    ///
    /// This can return an `Err(ScopeError)` if there are no more cached scopes.
    pub fn restore_scopes(&mut self) -> Result<(), ScopeError> {
        let Some(cache) = self.cached_scopes.pop() else {
            return Err(ScopeError::OutOfCachedScopes);
        };

        self.restore_scope(cache);
        Ok(())
    }

    /// Delete the specified number of scopes. This is non-reversible, unlike caching.
    pub fn delete_scopes(&mut self, target: u64) -> Result<(), ScopeError> {
        let mut scopes = 0;
        let mut bin = Vec::new();

        while scopes < target {
            // If the stack is emptied before the target, restore
            let Some(var) = self.current_stack.pop() else {
                self.restore_scope(bin);
                return Err(ScopeError::OutOfVars);
            };

            if var.is_divider() {
                scopes += 1;
            }

            bin.push(var);
        }

        Ok(())
    }

    /// Pop until a `Variable::StackDivider(Some(ScopeType::Function(_)))` is found. Used to remove unused values
    /// after a function returns.
    /// As with all other operations, if it runs out of variables to pop, it restores all of the
    /// impacted vars and returns an `Err()`
    pub fn return_cleanup(&mut self) -> Result<(), ScopeError> {
        let mut bin = Vec::new();

        loop {
            let Some(var) = self.current_stack.pop() else {
                self.restore_scope(bin);
                return Err(ScopeError::OutOfVars);
            };

            match var {
                Variable::StackDivider(Some(ScopeType::Function(_))) => return Ok(()),
                _ => bin.push(var),
            }
        }
    }

    /// Check if the last entry in the stack is a `Variable::Return`. If it is, return the inner
    /// value of it.
    pub fn is_return(&mut self) -> Option<Option<Object<'a>>> {
        let last = self.current_stack.pop().unwrap();
        if let Variable::Return(value) = last.clone() {
            self.current_stack.push(last);
            return Some(value);
        } else {
            self.current_stack.push(last);
            return None;
        }
    }

    /// Used by `restore_scopes()` and `cache_scopes()`
    fn restore_scope(&mut self, mut scope: Vec<Variable<'a>>) {
        while let Some(var) = scope.pop() {
            self.current_stack.push(var);
        }
    }
}

/// # ScopeError
/// This is an enum representing various things that can go wrong while working with scopes.
#[derive(Debug)]
pub enum ScopeError {
    /// Represents having no more cached scopes to restore.
    OutOfCachedScopes,

    /// Represents having no more variables to pop before reaching some quota
    OutOfVars,
}

impl fmt::Display for ScopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let explanation = match self {
            Self::OutOfCachedScopes => "ran out of cached scopes, so can't restore a scope",
            Self::OutOfVars => {
                "ran out of vars to cache before the target number of scopes to cache was met"
            }
        };

        write!(f, "{explanation}")
    }
}

impl Error for ScopeError {}

#[cfg(test)]
mod tests {
    use super::*;

    //Put a bunch of garbage on the stack and make sure it looks up only for first value with a
    //given name
    #[test]
    fn lookup() {
        let mut stack = ScopeStack::new();

        // fill the stack with garbage
        stack.push(Variable::Var {
            name: String::from("pi"),
            value: Object::Integer(10),
        });
        stack.push(Variable::StackDivider(None));
        stack.push(Variable::Var {
            name: String::from("siiiix_seeeeeven"),
            value: Object::Integer(67),
        });

        // push the value we actually want
        stack.push(Variable::Var {
            name: String::from("pi"),
            value: Object::Float(3.1415926535),
        });

        let lookup = stack.lookup(&mut String::from("pi")).unwrap();
        let Variable::Var { value, .. } = lookup else {
            panic!("Lookup was a StackDivider");
        };
        assert_eq!(value.float(), Some(3.1415926535));
    }
}
