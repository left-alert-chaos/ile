use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("ile: no arguments were given");
    } else {
        let time = Instant::now();
        match ile::ast_from_file(args[1].clone()) {
            Ok(ast) => {
                println!("\n\nDone! AST:\n{ast:#?}");
            }
            Err(e) => println!("\n\n{e}"),
        }
        eprintln!("\n\nElapsed time to generate AST: {:?}", time.elapsed());
    }
}
