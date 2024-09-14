use std::env::current_dir;

use crate::schema::SchemaOpt;

use super::Generate;

fn run_on_fixture(path: &str) {
    let plan = current_dir()
        .unwrap()
        .join("tests")
        .join("fixtures")
        .join("plans")
        .join(path);

    let generate = Generate {
        schema: SchemaOpt { plan },
        output: None,
    };

    generate.run().unwrap();

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

#[test]
fn basic() {
    run_on_fixture("basic.yaml");
}

#[test]
fn folder() {
    run_on_fixture("folder");
}
