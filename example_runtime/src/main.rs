use ile::*;
use ile::module::*;

fn build_lib<'a>() -> Library<'a> {
    let mut lib = Library::new("example_runtime");
    lib.add_function(&print_function, signature!("string"), "print");
    lib
}

fn print_function(args: FunctionSignature<'_>) -> FunctionResult<'_> {
    if args.len() != 1 {
        return Err(Error::new_rust("example_runtime.print_function() takes one argument"));
    }
    println!("{}", args[0]);
    Ok(None)
}

fn main() -> Result<(), Error> {
    let mut extension = ile::ast_from_file("extension.il")?;
    extension.add_library(build_lib());
    extension.walk_as_mod(true)?;
    Ok(())
}
