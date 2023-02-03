use std::{env, fs, path::Path};

use assert_cmd::Command;

const HELP_STR: &str = "resume related commands

Usage: cap resume [OPTIONS] <COMMAND>

Commands:
  list  list resume entries in your config file
  gen   generate new resume
  help  Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  Config file
  -h, --help             Print help
";

const LIST_HELP_STR: &str = "list resume entries in your config file

Usage: cap resume list [OPTIONS]

Options:
  -c, --config <CONFIG>  Config file
  -h, --help             Print help
";

const GEN_HELP_STR: &str = "generate new resume

Usage: cap resume gen [OPTIONS] --name <NAME>

Options:
  -c, --config <CONFIG>      Config file
  -n, --name <NAME>          resume key in your config file
  -t, --template <TEMPLATE>  template name or local path to the your custom template file
  -o, --out <OUT>            output pdf file name eg. my_resume.pdf
  -h, --help                 Print help
";

const LIST_ERR_STR: &str = "Error: No such file or directory (os error 2)\n";

const LIST_STR: &str = "+---+-------------+
| * | my_resume_1 |
+---+-------------+
|   | my_resume_2 |
+---+-------------+
";

#[test]
fn help() {
    Command::cargo_bin("cap")
        .unwrap()
        .args(&["resume", "--help"])
        .assert()
        .success()
        .stdout(HELP_STR);
}

#[test]
fn list_help() {
    Command::cargo_bin("cap")
        .unwrap()
        .args(&["resume", "list", "--help"])
        .assert()
        .success()
        .stdout(LIST_HELP_STR);
}

#[test]
fn gen_help() {
    Command::cargo_bin("cap")
        .unwrap()
        .args(&["resume", "gen", "--help"])
        .assert()
        .success()
        .stdout(GEN_HELP_STR);
}

#[test]
fn list_err() {
    Command::cargo_bin("cap")
        .unwrap()
        .args(&["resume", "list"])
        .assert()
        .failure()
        .stderr(LIST_ERR_STR);
}

#[test]
fn list() {
    Command::cargo_bin("cap")
        .unwrap()
        .args(&["resume", "list", "--config", "tests/test.config.yml"])
        .assert()
        .success()
        .stdout(LIST_STR);
}

#[test]
fn gen() {
    let f = env::temp_dir().join("my_resume.pdf");
    fs::remove_file(&f).ok();

    let path = &f.to_str().unwrap().to_string();
    Command::cargo_bin("cap")
        .unwrap()
        .args(&[
            "resume",
            "gen",
            "--config",
            "tests/test.config.yml",
            "--name",
            "my_resume_1",
            "--out",
            path,
        ])
        .assert()
        .success();

    assert!(Path::new(path).is_file());
}
