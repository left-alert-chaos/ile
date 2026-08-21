use ile;

use std::fs;

#[test]
fn run_all_examples() {
    fs::write("examples_output.txt", "").unwrap();
    for example_name in fs::read_dir("examples").unwrap() {
        // get file name
        let name = example_name.unwrap().path();
        let name = name.to_str().unwrap();
        log(format!("reading {name}...\n"));

        // skip because testing hangs while waiting for input
        if name.ends_with("echo.il") {
            log(format!("skipping {name} because it uses input()"));
            continue;
        }

        // attempt to walk it
        let mut ast = ile::ast_from_file(name).unwrap();
        ast.walk_as_mod(true).unwrap();
        log(format!("Successfully read and ran {name}!\n\n"));
    }
}

fn log(msg: impl ToString) {
    let mut contents = String::from_utf8(fs::read("examples_output.txt").unwrap()).unwrap();
    contents.push_str(msg.to_string().as_str());
    fs::write("examples_output.txt", contents).unwrap();
}
