extern crate assert_cmd;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_not_found() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.arg("tests/doesnotexist.yml");
    cmd.assert()
        .failure()
        .code(1)
        .stderr(predicate::str::similar(
            "Error: Could not read file `tests/doesnotexist.yml`\nInfo: caused by No such file or directory (os error 2)\n",
        ));
}

#[test]
fn file_invalid() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.arg("tests/.invalid-gitlab-ci.yml");
    cmd.assert()
        .failure()
        .code(1)
        .stderr(predicate::str::similar(
            "Error: tests/.invalid-gitlab-ci.yml is invalid\n- jobs config should contain at least one visible job\n",
        ));
}

#[test]
fn host_invalid() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.arg("tests/.gitlab-ci.yml").arg("--host").arg("yo");
    cmd.assert()
        .failure()
        .code(1)
        .stderr(predicate::str::similar(
            "Error: relative URL without a base\n",
        ));
}

#[test]
fn file_valid() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.arg("tests/.gitlab-ci.yml");
    cmd.assert()
        .success()
        .code(0)
        .stdout(predicate::str::similar("tests/.gitlab-ci.yml is valid\n"));
}
