# Hello, world!
This is the first chapter of the Ile language documentation.

I'm not very good at pretty documentation writing, so I'll get straight to it. The first step to writing Ile code without a 3rd-party runtime is downloading the interpreter with the directions in the main `README.md` file. Once you've got it running, come back here.

## Greetings!
Writing to the standard output in Ile is fairly simple, and it steals Rust's `println` name. Here's the full snippet:

```
println("Hello, world!");
```

Save that to a `hello.il` and run it with `ile hello.il`. Congratulations! You've written some Ile! However, we can take this further. Let's define some variables!

## Variables
Variables in Ile are defined with the `let` keyword. All variables are mutable. Once you have initialized a variable with `let`, you can re-assign to it with and equals sign. Example:

```
let x = 5;
x = 1;
```

The above example creates a new variable `x` and assigns it the value `5`. It then re-assigns it the value `1`. Simple, right?
