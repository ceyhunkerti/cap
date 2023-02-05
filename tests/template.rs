use assert_cmd::Command;

const HELP_STR: &str = "html resume template commands

Usage: cap template [OPTIONS] <COMMAND>

Commands:
  list  list available templates
  help  Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  Config file
  -h, --help             Print help
";

const LIST_HELP_STR: &str = "list available templates

Usage: cap template list [OPTIONS]

Options:
  -c, --config <CONFIG>  Config file
  -h, --help             Print help
";

const LIST_STR: &str = "+-----------+
| basic     |
+-----------+
| basicplus |
+-----------+
";

#[test]
fn help() {
    Command::cargo_bin("cap")
        .unwrap()
        .args(&["template", "--help"])
        .assert()
        .success()
        .stdout(HELP_STR);
}

#[test]
fn list_help() {
    Command::cargo_bin("cap")
        .unwrap()
        .args(&["template", "list", "--help"])
        .assert()
        .success()
        .stdout(LIST_HELP_STR);
}

#[test]
fn list() {
    Command::cargo_bin("cap")
        .unwrap()
        .args(&["template", "list"])
        .assert()
        .success()
        .stdout(LIST_STR);
}
