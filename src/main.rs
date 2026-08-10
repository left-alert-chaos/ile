use ile::ast_from_str;

fn main() {
    let code = "println(\"hello world!\");";
    match ast_from_str(code) {
        Ok(ast) => {
            println!("\n\n\nDone! AST:\n{ast:#?}");
        }
        Err(e) => println!("\n\n{e}"),
    }
}
