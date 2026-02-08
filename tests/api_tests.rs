use envsafe_cli::api::{
    ApiClient, EnvVariable, EnvironmentResponse, Project, ProjectsResponse, User, Workspace,
    WorkspacesResponse,
};

#[test]
fn test_user_deserialization() {
    let json = r#"{
        "id": "user123",
        "name": "John Doe",
        "email": "john@example.com",
        "lastLoginAt": "2024-01-01T00:00:00Z"
    }"#;

    let user: User = serde_json::from_str(json).unwrap();
    assert_eq!(user.id, "user123");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "john@example.com");
    assert_eq!(user.last_login, Some("2024-01-01T00:00:00Z".to_string()));
}

#[test]
fn test_workspaces_response_deserialization() {
    let json = r#"{
        "workspaces": [
            {
                "id": "ws1",
                "name": "Workspace 1",
                "slug": "workspace-1",
                "description": "Test workspace",
                "imageUrl": "https://example.com/image.png",
                "role": "OWNER",
                "projectCount": 5
            }
        ],
        "count": 1
    }"#;

    let response: WorkspacesResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.count, 1);
    assert_eq!(response.workspaces.len(), 1);

    let workspace = &response.workspaces[0];
    assert_eq!(workspace.id, "ws1");
    assert_eq!(workspace.name, "Workspace 1");
    assert_eq!(workspace.slug, Some("workspace-1".to_string()));
    assert_eq!(workspace.project_count, Some(5));
}

#[test]
fn test_projects_response_deserialization() {
    let json = r#"{
        "projects": [
            {
                "id": "proj1",
                "name": "Project 1",
                "slug": "project-1",
                "description": "Test project",
                "environments": ["development", "staging", "production"],
                "updatedAt": "2024-01-01T00:00:00Z"
            }
        ],
        "count": 1
    }"#;

    let response: ProjectsResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.count, 1);
    assert_eq!(response.projects.len(), 1);

    let project = &response.projects[0];
    assert_eq!(project.id, "proj1");
    assert_eq!(project.name, "Project 1");
    assert_eq!(project.slug, Some("project-1".to_string()));
    assert_eq!(project.environments.as_ref().unwrap().len(), 3);
}

#[test]
fn test_environment_response_deserialization() {
    let json = r#"{
        "project": "test-project",
        "environment": "development",
        "variables": {
            "DATABASE_URL": "postgresql://localhost/db",
            "API_KEY": "secret123",
            "DEBUG": "true"
        },
        "count": 3
    }"#;

    let response: EnvironmentResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.project, "test-project");
    assert_eq!(response.environment, "development");
    assert_eq!(response.count, 3);
    assert_eq!(response.variables.len(), 3);
    assert_eq!(
        response.variables.get("DATABASE_URL").unwrap(),
        "postgresql://localhost/db"
    );
    assert_eq!(response.variables.get("API_KEY").unwrap(), "secret123");
    assert_eq!(response.variables.get("DEBUG").unwrap(), "true");
}

#[test]
fn test_env_variable_creation() {
    let var = EnvVariable {
        key: "TEST_KEY".to_string(),
        value: "test_value".to_string(),
        updated_at: None,
    };

    assert_eq!(var.key, "TEST_KEY");
    assert_eq!(var.value, "test_value");
    assert!(var.updated_at.is_none());
}

#[test]
fn test_api_client_creation() {
    let _client = ApiClient::new("https://api.example.com".to_string());

    // Client should be created successfully
    // We can't test much without making actual HTTP requests
    assert!(true);
}

#[test]
fn test_workspace_missing_optional_fields() {
    let json = r#"{
        "id": "ws1",
        "name": "Minimal Workspace"
    }"#;

    let workspace: Workspace = serde_json::from_str(json).unwrap();
    assert_eq!(workspace.id, "ws1");
    assert_eq!(workspace.name, "Minimal Workspace");
    assert!(workspace.slug.is_none());
    assert!(workspace.description.is_none());
    assert!(workspace.image_url.is_none());
    assert!(workspace.role.is_none());
    assert!(workspace.project_count.is_none());
}

#[test]
fn test_project_missing_optional_fields() {
    let json = r#"{
        "id": "proj1",
        "name": "Minimal Project"
    }"#;

    let project: Project = serde_json::from_str(json).unwrap();
    assert_eq!(project.id, "proj1");
    assert_eq!(project.name, "Minimal Project");
    assert!(project.slug.is_none());
    assert!(project.description.is_none());
    assert!(project.environments.is_none());
    assert!(project.updated_at.is_none());
}
