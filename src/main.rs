fn main() {
    let args: Vec<String> = std::env::args().into_iter().collect();

    if args.len() < 2 {
        eprintln!("ile: no arguments were given");
    } else {
        match ile::ast_from_file(args[1].clone()) {
            Ok(ast) => {
                println!("\n\nDone! AST:\n{ast:#?}");
            }
            Err(e) => println!("\n\n{e}"),
        }
    }
}
