use ile::*;
use std::time::Instant;

fn main() -> Result<(), error::Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&String::from("-i")) {
        println!("Objects are {} bytes. Nodes are {} bytes.", size_of::<ile::Object>(), size_of::<ile::Node>());
        return Ok(());
    }

    if args.len() < 2 {
        eprintln!("ile: no arguments were given");
    } else {
        let time = Instant::now();
        match ile::ast_from_file(args[1].clone()) {
            Ok(mut ast) => {
                println!("{ast:#?}");
                let NodeType::Root { name, mut stack, statements } = ast.ntype.clone() else {
                    unreachable!();
                };
                ast.walk(&mut stack)?;
                ast = Node {
                    token: None,
                    ntype: NodeType::Root { name, stack, statements }
                };
                println!("\n\nDone! AST:\n{ast:#?}");
            }
            Err(e) => println!("\n\n{e}"),
        }
        eprintln!("\n\nElapsed time to generate AST: {:?}", time.elapsed());
    }

    Ok(())
}
