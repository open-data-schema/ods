use snapbox::{
    cmd::{cargo_bin, Command},
    data::Data,
    utils::current_dir,
};

fn run_on_fixture(path: &str, args: &[&str], fail: bool) {
    let plan = current_dir!().join("fixtures").join("lint").join(path);
    let snapshot = current_dir!().join("snapshots").join("lint").join(path);

    let assert = Command::new(cargo_bin!("ods"))
        .args(&["--color", "always", "lint"])
        .args(args)
        .arg(plan)
        .assert();

    let assert = if fail {
        assert.failure()
    } else {
        assert.success()
    };

    assert
        .stderr_eq(Data::read_from(&snapshot.join("stderr.txt"), None))
        .stdout_eq(Data::read_from(&snapshot.join("stdout.txt"), None));
}

#[test]
fn non_existent() {
    run_on_fixture("non_existent", &[], true);
}

#[test]
fn empty() {
    run_on_fixture("empty.yaml", &[], true);
}

#[test]
fn change_levels() {
    run_on_fixture("change_levels", &[], true);
}
