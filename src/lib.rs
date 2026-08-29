//! # ile
//! Ile is the Integrated Language for Extensions. It's an embedded scripting language with tight
//! bindings to Rust, intended for use as a standalone language and for making extensions and
//! plugins for a larger platform.

pub mod ast;
pub mod error;
pub mod ilestd;
pub mod interface;
pub mod object;
pub mod scope;
pub mod seaweed;

pub use ast::*;
pub use error::*;
pub use ilestd::*;
pub use interface::*;
pub use object::*;
pub use scope::*;
