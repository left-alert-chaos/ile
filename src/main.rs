use ile::ast_from_str;

//println(\"hello world!\");
fn main() {
    let code = "println(get_text(), 5); let x = 5;";
    match ast_from_str(code) {
        Ok(ast) => {
            println!("\n\n\nDone! AST:\n{ast:#?}");
        }
        Err(e) => println!("\n\n{e}"),
    }
}
