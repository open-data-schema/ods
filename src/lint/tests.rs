use std::env::current_dir;

use crate::schema::SchemaOpt;

use super::Lint;

fn run_on_fixture(path: &str) {
    let plan = current_dir()
        .unwrap()
        .join("tests")
        .join("fixtures")
        .join("lints")
        .join(path);

    let lint = Lint {
        schema: SchemaOpt { plan },
    };

    lint.run().unwrap();

    // TODO: Diff and compare the output
}

#[test]
#[should_panic(expected = "No such file or directory")]
fn non_existent() {
    run_on_fixture("non_existent");
}

#[test]
#[should_panic(expected = "missing field `events`")]
fn empty() {
    run_on_fixture("empty.yaml");
}
