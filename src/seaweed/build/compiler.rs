//! # compiler
//! This module holds the data structures and helpers used to compile an AST.

use crate::*;

/// # Compiler
/// This holds all information needed to compile a program to bytecode.
#[derive(Clone, Debug)]
pub struct Compiler<'a> {
    /// Represents the in-progress list of instructions the compiler will generate
    instructions: Vec<Instruction<'a>>,

    /// The syntax tree this compiler is compiling
    tree: Node<'a>,

    /// The variable number it's on.
    /// This keeps track of what to call variables. For instance, if you define a
    /// variable `x` and then a variable `y`, these are compiled as `v0` and `v1`.
    /// That number is what this stores.
    var_num: usize,
}

impl<'a> Compiler<'a> {
    /// Creates a new compiler for the given AST and var_num. The var_num is necessary because the
    /// compiler can create children if a module is imported, and we don't want name collisions.
    pub fn new(tree: Node<'a>, var_num: usize) -> Self {
        Self {
            instructions: Vec::new(),
            tree,
            var_num,
        }
    }
}

/// # Instruction
/// This represents one instruction. It can be to create an array, move a value, call a function, or
/// anything else bytecode can do as a CPU instruction.
#[derive(Clone, Debug)]
pub enum Instruction<'a> {
    /// Represents a `mov` instruction.
    Mov {
        source: Location<'a>,
        to: Location<'a>,
    },

    /// Represents an instruction to take register values and combine into an array
    Array,

    /// Represents an instruction to clear a register's object (set it to None)
    Clear,

    /// Represents a return instruction
    Return(Option<Location<'a>>),

    /// Represents an instruction to create a new variable
    Let {
        name: String,
        source: Location<'a>,
    },

    /// Represents pushing a `StackDivider` onto the stack
    Divide(Option<Vec<String>>),

    /// Represents removing all entries from stack until a `StackDivider` is found
    End,

    /// Represents calling a function
    Call(Location<'a>),
}

/// # Location
/// This represents anywhere a value can come from: a variable or any type of register.
#[derive(Clone, Debug)]
pub enum Location<'a> {
    /// Represents a generic register--anything starting with "r"
    GenericRegister(usize),

    /// Represents a variable
    Var(String),

    /// Represents the result register; function returns and arrays go here, etc
    Result,

    /// Represents a literal declaration
    Literal(Object<'a>),

    /// Represents the `op1` register that holds the first arm of an operation
    Operation1,

    /// Represents the `op2` register that holds the second arm of an operation
    Operation2,
}
