use ile::*;

use std::io;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&String::from("-i")) {
        println!(
            "Objects are {} bytes. Nodes are {} bytes.",
            size_of::<ile::Object>(),
            size_of::<ile::Node>()
        );
        return Ok(());
    }

    if args.len() < 2 {
        repl()
    } else {
        let mut ast = ile::ast_from_file(args[1].clone())?;
        ast.walk_as_mod(true)
    }
}

fn repl() -> Result<(), Error> {
    println!("Welcome to the Ile REPL! Type the lines of your program, and then type 'exit' when you're done!");

    let mut code = String::new();
    loop {
        let mut buffer = String::new();
        if io::stdin().read_line(&mut buffer).is_err() {
            break;
        }
        buffer = buffer.trim().to_string();
        if buffer.as_str() == "exit" {
            break;
        }
        code.push('\n');
        code.push_str(buffer.as_str());
    }

    ast_from_str(code)?.walk_as_mod(true)
}
