# ile
<img src="ile.png">

Ile is the Interpreted Language for Extensions. It's an embedded scripting language with tight binding to Rust.

Here's a Hello World snippet:

```
println("Hello, world!");
```

As you can see, syntax-wise, it's not unlike Rust.

If you haven't used Ile before, take a look at the files in the `examples` directory for some, well, examples! The [documentation](https://github.com/left-alert-chaos/ile/blob/main/docs/lang/README.md) is also very useful.

# Quick start
*_WARNING: Currently, the GitHub releases are broken on Windows. If you are on windows, please install with Cargo._*

To get started with Ile, you can either download a released binary or install it with Cargo. If you're on x86 Windows or Linux, I'd suggest downloading a binary from the [releases](https://github.com/left-alert-chaos/ile/releases). The `zip` files in the releases contain a binary called `ile` or `ile.exe`.

## Installing with Cargo
If you're on an ARM machine or Mac, you'll have to install it with Cargo. To determine if you [have Cargo installed](https://doc.rust-lang.org/cargo/getting-started/installation.html), run this command in your terminal:

```shell
cargo --version
```

If something prints, you're good to go! Now install Ile with:

```shell
cargo install ile
```

Now you should be able to run it with the `ile` command. If you can't, it's probably because Cargo's `bin` directory isn't in your `PATH`. To temporarily add it on a Mac or on Linux, run this command:

```shell
export PATH="$PATH:~/.cargo/bin/"
```

# Usage
To run the interpreter with a source file, set the file to the interprer's argument, like `ile my_file.il`. If no argument is supplied, it enters REPL mode, where you can type your program out manually.

In REPL mode, you can load and run your program by typing "exit".

# Features
Here are some things Ile does well:

- Simple Rust API for easy integration into existing systems
- Super-simple memory management without a heavy garbage collector or confusing borrow checker
- Easy-to-learn syntax: if you've programmed before, Ile is a breeze
- Lightweight: a simple infinite loop uses 60.3 megabytes of RAM in JavaScript (Node), but only 2.8 in Ile

# Dependencies
Ile has no dependencies! It doesn't require any other crates, and you don't need any special system packages. The only prerequisite is Cargo to build it. It'll probably work with any reasonably-recent `rustc` version.

# How it works
Ile is a traditional tree-walking interpreter. When you give it a source file, it first reads all characters in the file and tokenizes them. The tokenizer (lexer in other languages) is responsible for transforming the irregular and unpredictable human-readable (hopefully!) code into a bunch of tokens. A token is a piece of information that represents a bit of source code. It could be one character, like an equals sign, or it could be a bunch, like a variable name. Once the tokenizer has finished, the tokens are given to the parser, which converts them into an abstract syntax tree (AST). Ile uses a recursive descent parser. It reads tokens to guess which ones go together, and what kind of statement they form. When statements contain other statements, it calls itself again to process the child statement. Each group of tokens is parsed into a node, which is an in-memory representation of a piece of logic. It could be a variable declaration, function call, import, or anything else you can do.

Once all the tokens have been parsed into a tree (one root node all other nodes are children of), execution can start. This is called "walking" the AST. Walking starts at the root node and does a depth-first search of all nodes and their children. For each node it encounters, it determines how to execute it based on its node type (`ntype`). If a node needs to walk children to get necessary values, like when a `let` statement is set to the result of a function call, its children are walked and their values are stored. If a node needs to make changes to the stack, it does it and finishes, returning a value if it should.

The previously-mentioned stack is a `Vec` of stack entries which can be anything stored in memory--variables, modules, or imports--which is searched in reversed order to determine which variable is referenced when a name is written.

# Documentation
All documentation for the language is available in the `docs/` directory. [Here's a link!](https://github.com/left-alert-chaos/ile/tree/main/docs/README.md)
