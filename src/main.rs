use ile::*;

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
        let mut ast = ile::ast_from_file(args[1].clone())?;
        return ast.walk_as_mod(true);
    }

    Ok(())
}
