# Errors
The example we have so far works, but it doesn't account for the fact that our extension might not give the right number of arguments. In the case we _do_ get a bad number of arguments, we need to be prepared so that nothing bad like a panic happens. That's where errors come in!

To raise an error, you return from your function with an `Err` holding an `Error`. These are pretty easy to create, so let's make our `print` function safe!

Put this snippet at the start of `print_function`:

```rust
if args.len() != 1 {
    return Err(Error::new_rust("example_runtime.print_function() takes one argument"));
}
```

Now, if you pass the wrong number of arguments to `print`, it raises and error instead of potentially panicking!
