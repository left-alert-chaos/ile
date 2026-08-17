use ile::*;
use std::time::Instant;

fn main() -> Result<(), error::Error> {
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
        eprintln!("ile: no arguments were given");
    } else {
        let time = Instant::now();
        match ile::ast_from_file(args[1].clone()) {
            Ok(mut ast) => {
                let NodeType::Root {
                    mut stack,
                    ..
                } = ast.ntype.clone()
                else {
                    unreachable!();
                };

                // include the standard library
                ile::ilestd::include::include(&mut stack);

                ast.walk(&mut stack)?;
            }
            Err(e) => println!("\n\n{e}"),
        }
        eprintln!("\n\nElapsed time to generate and walk AST: {:?}", time.elapsed());
    }

    Ok(())
}
