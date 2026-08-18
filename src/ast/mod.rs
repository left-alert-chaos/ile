//! # ast
//! This module holds code to represent and build an Abstract Syntax Tree.

pub mod arguments;
pub mod build;
pub mod node;
mod walk;

pub use arguments::*;
pub use build::*;
pub use node::*;

use crate::*;

/// Read a file at the given path and perform the entire module pipeline:
/// - Tokenize the contents
/// - Parse an abstract syntax tree from the tokens
pub fn ast_from_file<'a>(path: impl ToString) -> Result<Node<'a>, Error> {
    let path = path.to_string();

    let chars = match std::fs::read(&path) {
        Ok(text) => text,
        Err(_) => {
            return Err(Error::new_parsing(
                None,
                format!("couldn't locate module '{path}'"),
            ));
        }
    };

    let text = match String::from_utf8(chars) {
        Ok(text) => text,
        Err(_) => {
            return Err(Error::new_parsing(
                None,
                format!("couldn't read module '{path}' as utf-8"),
            ));
        }
    };

    let tokens = tokenize(text, Some(path.clone()))?;

    Ok(Parser::build_root(tokens, path)?)
}

/// Tokenize the given source code and parse it into an AST.
/// Not recommended, because it doesn't record the file the code came from.
pub fn ast_from_str<'a>(code: impl ToString) -> Result<Node<'a>, String> {
    let tokens = tokenize(code, None)?;
    Ok(Parser::build_root(tokens, String::from("unknown"))?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_build(code: &str) -> Node<'_> {
        let ast = ast_from_str(code).unwrap();
        println!("{ast:#?}");
        ast
    }

    fn test_var<'a>(name: impl ToString, mut ast: Node<'a>) -> Object<'a> {
        let mut stack = ScopeStack::new();
        ast.walk(&mut stack).unwrap();
        let name = name.to_string();
        let Some(entry) = stack.lookup(&name) else {
            panic!("stack has no entry '{name}'");
        };
        let Variable::Var { value, .. } = entry else {
            panic!("retrieved stack entry wasn't a var");
        };
        value.clone()
    }

    #[test]
    fn parse_hello_world_wip() {
        let hello_world = "println(\"Hello, world!\");";
        test_build(hello_world);
    }

    #[test]
    fn parse_nested_functions_wip() {
        let code = "func(func());";
        test_build(code);
    }

    #[test]
    fn parse_let_wip() {
        let code = "let x = 5;";
        test_build(code);
    }

    #[test]
    fn parse_let_and_call() {
        let code = "func();
        let x = 5;
        func2();";
        test_build(code);
    }

    #[test]
    fn parse_if_else_if_else() {
        let code = "if 15 > 0 {
            do_something();
        } else if something() {
            somethingelse()
        } else {}";
        test_build(code);
    }

    #[test]
    fn parse_function_def() {
        let code = "let func = () {
            otherfunc();
        };";
        test_build(code);
    }

    #[test]
    fn parse_for() {
        let code = "for let x = do_something_i_guess(); {
            asdf();
        }";
        test_build(code);
    }

    #[test]
    fn parse_while() {
        let code = "while let x = do_another_thing(); {
            abcd();
        }";
        test_build(code);
    }

    #[test]
    fn parse_import() {
        let code = "import \"module.il\";";
        test_build(code);
    }

    #[test]
    fn parse_datatype() {
        let code = "datatype DataType {
            let attr1 = 3;
            let attr2 = \"string\";
            let method1 = () {
                println(\"hello\");
            };
        }";
        test_build(code);
    }

    #[test]
    fn parse_array() {
        let code = r#"let arr = [
            5,
            "a string if you want",
            func(),
        ];"#;
        test_build(code);
    }

    #[test]
    fn add_ints() {
        let code = r#"let x = 5;
        let y = 1;
        let z = x + y;"#;
        let ast = test_build(code);
        assert_eq!(test_var("z", ast).integer().unwrap(), 6);
    }

    #[test]
    fn nested_attribute_lookup() {
        let code = "
        datatype Inner {
            let attr = 6;
        }

        datatype Middle {
            let attr = Inner();
        }

        datatype Outer {
            let attr = Middle();
        }

        let o = Outer();
        let x = o.attr.attr.attr;";
        let ast = test_build(code);
        assert_eq!(test_var("x", ast).integer().unwrap(), 6);
    }

    #[test]
    fn break_loop() {
        let code = "
        let x = 0;
        while true {
            x = x + 1;
            if x > 4 {
                break;
            }
        }
        ";
        let ast = test_build(code);
        assert_eq!(test_var("x", ast).integer().unwrap(), 5);
    }
}
