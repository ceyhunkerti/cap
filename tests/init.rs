use std::{env, fs, path::Path};

use assert_cmd::Command;

const HELP_STR: &str = "initialize config file

Usage: cap init [OPTIONS]

Options:
  -c, --config <CONFIG>  Config file
  -p, --path <PATH>      path to config file. eg. [~/documents, ./my_config.yml]
  -h, --help             Print help
";

#[test]
fn help() {
    Command::cargo_bin("cap")
        .unwrap()
        .args(&["init", "--help"])
        .assert()
        .success()
        .stdout(HELP_STR);
}

#[test]
fn init() {
    let f = env::temp_dir().join("cap.yml");
    fs::remove_file(&f).ok();

    let path = &f.to_str().unwrap().to_string();

    Command::cargo_bin("cap")
        .unwrap()
        .args(&["init", "--path", path])
        .assert()
        .success();

    assert!(Path::new(path).is_file());
    let content = fs::read_to_string(&f).expect("Unable to read template");
    assert!(!content.is_empty());
}
