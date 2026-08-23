# Loading and running a module
The first step to interacting with extensions is being able to execute them in the first place. Here, we'll write some simple code to load and run an extension.

## Loading an extension

To load an extension, you just need the path to its source file. Here's the code in `example_runtime`:

```rust
use ile::*;

fn main() -> Result<(), Error> {
    let extension = ile::ast_from_file("extension.il")?;
    Ok(())
}
```

Here, we're telling Ile to tokenize and parse `extension.il` into an AST. This can fail, so the function that handles extensions should return a `Result<(), ile::Error>`. Because this is a really simple package, this is just the main function. In larger projects, you'd obviously put this in its own function or even module.

## Walking the extension
Executing an abstract syntax tree is called _walking_ it. This sounds complicated, but fortunately Ile makes this really easy! Here's the example:

```rust
use ile::*;

fn main() -> Result<(), Error> {
    let mut extension = ile::ast_from_file("extension.il")?;
    extension.walk_as_mod(true)?;
    Ok(())
}
```

This code alone is now enough to load and run an extension!
