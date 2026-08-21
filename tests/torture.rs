//! Responsible for running the torture test suite

use ile;

#[test]
fn torture() {
    let mut ast = ile::ast_from_file("tests/torture/main.il").unwrap();
    ast.walk_as_mod(true).unwrap();
}
