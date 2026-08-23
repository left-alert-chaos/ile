# Writing an API
So far, we have a package that can load and run an Ile extension. However, we can't _interact_ with that extension yet. Here, we'll write a simple `Library` to make that happen.

## Libraries
Interoperation between Ile and Rust revolves around the definition of Rust functions that take arbitrary Ile arguments. It is purely functional; there is, at time of writing, no way of wrapping a Rust type and letting Ile interact with it. However, you _can_ read and interact with Ile objects from Rust. It is worth revisiting Ile's [memory management strategy](../lang/memory.md) if you aren't familiar with it.

To provide a bundle of functions that Ile can interact with, you define a `Library`. This is a helper struct to make it easy to include your modules inside an extension's scope. This is how the standard library was written.

Here's a simple `Library`:

```rust
use ile::*;
use ile::module::*;

fn build_lib<'a>'() -> Library<'a> {
    let mut lib = Library::new::("example_runtime");
    lib
}
```

However, this isn't enough. We need to make our library _do_ something. Therefore, we need to add a function to it. The function we add is written in Rust, and has to take a `FunctionSignature` as an argument and return a `FunctionResult` (an alias of `Result<Option<Object<'_>>, Error>`).

Here's our function (we'll add it to the library in a second.):

```rust
fn print_function(args: FunctionSignature<'_>) -> FunctionResult<'_> {
    println!("{}", args[0]);
    Ok(None)
}
```

Now, we add it to the module:

```rust
fn build_lib<'a>() -> Library<'a> {
    let mut lib = Library::new("example_runtime");
    lib.add_function(&print_function, signature!("string"), "print");
    lib
}
```

Now, we add the `Library` to the scope of our extension. Back in `main`, we add this before calling `walk_as_mod()`:

```rust
extension.add_library(build_lib());
```

We can now run `example_runtime` and see a message!
