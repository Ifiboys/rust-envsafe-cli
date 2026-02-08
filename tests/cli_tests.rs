use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_version() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("envsafe 0.1.0"));
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
        .stdout(predicate::str::contains("Se connecter avec un token API"));
}

#[test]
fn test_pull_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["pull", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Télécharger les variables"));
}

#[test]
fn test_push_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["push", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Envoyer les variables"));
}

#[test]
fn test_init_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["init", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Configurer un projet"));
}

#[test]
fn test_link_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["link", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Lier ce répertoire"));
}

#[test]
fn test_whoami_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["whoami", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Afficher l'utilisateur connecté"));
}

#[test]
fn test_logout_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["logout", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Se déconnecter"));
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
        .stdout(predicate::str::contains("Gérer la configuration"));
}

#[test]
fn test_lang_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["lang", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Changer la langue"));
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
        .stdout(predicate::str::contains("Exécuter une commande"));
}

#[test]
fn test_list_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Lister les projets"));
}

#[test]
fn test_create_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["create", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Créer un nouveau projet"));
}

#[test]
fn test_select_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["select", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Sélectionner un projet"));
}

#[test]
fn test_watch_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["watch", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("surveillance"));
}

#[test]
fn test_rotate_help() {
    Command::new(env!("CARGO_BIN_EXE_envsafe"))
        .args(&["rotate", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rotation"));
}
