//! # vebagu
//! VeBaGu stands for Very Bad Gui. It is a dumb way of displaying a GUI by sending text to display
//! to a Python script that draws it in a Tkinter window.

use crate::*;

/// Build VeBaGu as a `module::Library`.
pub fn build<'a>() -> module::Library<'a> {
    let mut vebagu = module::Library::new("vebagu");

    vebagu
}
