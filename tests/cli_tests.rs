use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_version() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("envsafe 0.2"));
}

#[test]
fn test_cli_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("EnvSafe CLI"))
        .stdout(predicate::str::contains("Commands:"));
}

#[test]
fn test_login_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["login", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Authenticate with EnvSafe"));
}

#[test]
fn test_pull_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["pull", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Download environment variables"));
}

#[test]
fn test_push_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["push", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Upload environment variables"));
}

#[test]
fn test_init_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["init", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialize a project"));
}

#[test]
fn test_link_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["link", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Link current directory"));
}

#[test]
fn test_whoami_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["whoami", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Show current user"));
}

#[test]
fn test_logout_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["logout", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Log out"));
}

#[test]
fn test_m2m_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["m2m", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Machine-to-Machine"));
}

#[test]
fn test_config_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["config", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Manage configuration"));
}

#[test]
fn test_lang_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["lang", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Change CLI language"));
}

#[test]
fn test_invalid_command() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .arg("invalid_command")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn test_run_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["run", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Run a command with injected variables",
        ));
}

#[test]
fn test_list_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("List projects"));
}

#[test]
fn test_create_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["create", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Create a new project"));
}

#[test]
fn test_select_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["select", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Select a project"));
}

#[test]
fn test_watch_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["watch", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Start real-time variable monitoring",
        ));
}

#[test]
fn test_rotate_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["rotate", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Manage secret rotation"));
}
