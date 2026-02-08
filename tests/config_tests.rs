use envsafe_cli::config::{Config, ProjectConfig};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_config_default() {
    let config = Config::default();
    assert_eq!(config.api_url, "https://www.envsafe.dev");
    assert_eq!(config.dashboard_url, "https://www.envsafe.dev");
    assert_eq!(config.language, "en");
    assert!(config.token.is_none());
    assert!(config.current_workspace.is_none());
    assert!(config.current_project.is_none());
}

#[test]
fn test_config_set_language() {
    let mut config = Config::default();
    assert_eq!(config.language, "en");

    config.language = "fr".to_string();
    assert_eq!(config.language, "fr");
}

#[test]
fn test_project_config_serialization() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".envsafe");

    let project_config = ProjectConfig {
        workspace_id: "workspace123".to_string(),
        project_id: "project456".to_string(),
        project_name: "Test Project".to_string(),
        project_slug: Some("test-project".to_string()),
    };

    // Save config
    let content = serde_json::to_string_pretty(&project_config).unwrap();
    fs::write(&config_path, content).unwrap();

    // Load config
    let loaded_content = fs::read_to_string(&config_path).unwrap();
    let loaded_config: ProjectConfig = serde_json::from_str(&loaded_content).unwrap();

    assert_eq!(loaded_config.workspace_id, "workspace123");
    assert_eq!(loaded_config.project_id, "project456");
    assert_eq!(loaded_config.project_name, "Test Project");
    assert_eq!(loaded_config.project_slug, Some("test-project".to_string()));
}

#[test]
fn test_project_config_backward_compatibility() {
    // Test that old configs without slug still work
    let json = r#"{
        "workspace_id": "ws1",
        "project_id": "proj1",
        "project_name": "Old Project"
    }"#;

    let config: ProjectConfig = serde_json::from_str(json).unwrap();
    assert_eq!(config.workspace_id, "ws1");
    assert_eq!(config.project_id, "proj1");
    assert_eq!(config.project_slug, None);
}
