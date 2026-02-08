use crate::config::Config;
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(rename = "lastLoginAt")]
    pub last_login: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspacesResponse {
    pub workspaces: Vec<Workspace>,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "imageUrl", default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(rename = "projectCount", default)]
    pub project_count: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectsResponse {
    pub projects: Vec<Project>,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub environments: Option<Vec<String>>,
    #[serde(rename = "updatedAt", default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentResponse {
    pub project: String,
    pub environment: String,
    pub variables: std::collections::HashMap<String, String>,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    pub id: String,
    pub name: String,
    pub variables: Vec<EnvVariable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVariable {
    pub key: String,
    pub value: String,
    pub updated_at: Option<String>,
}

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub fn from_config(config: &Config) -> Self {
        Self::new(config.api_url.clone())
    }

    pub async fn get_user(&self, token: &str) -> Result<User> {
        let url = format!("{}/api/v1/user/me", self.base_url);

        let response = self.client.get(&url).bearer_auth(token).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get user info: {}", response.status());
        }

        let user = response.json::<User>().await?;
        Ok(user)
    }

    pub async fn get_workspaces(&self, token: &str) -> Result<Vec<Workspace>> {
        let url = format!("{}/api/v1/workspaces", self.base_url);

        let response = self.client.get(&url).bearer_auth(token).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get workspaces: {}", response.status());
        }

        let workspaces_response = response.json::<WorkspacesResponse>().await?;
        Ok(workspaces_response.workspaces)
    }

    pub async fn get_projects(&self, token: &str, workspace_id: &str) -> Result<Vec<Project>> {
        let url = format!(
            "{}/api/v1/workspaces/{}/projects",
            self.base_url, workspace_id
        );

        let response = self.client.get(&url).bearer_auth(token).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get projects: {}", response.status());
        }

        let projects_response = response.json::<ProjectsResponse>().await?;
        Ok(projects_response.projects)
    }

    pub async fn get_environment(
        &self,
        token: &str,
        project_id: &str,
        env_name: &str,
    ) -> Result<Environment> {
        let url = format!(
            "{}/api/v1/projects/{}/{}",
            self.base_url, project_id, env_name
        );

        let response = self.client.get(&url).bearer_auth(token).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get environment: {}", response.status());
        }

        let env_response = response.json::<EnvironmentResponse>().await?;

        let variables: Vec<EnvVariable> = env_response
            .variables
            .into_iter()
            .map(|(key, value)| EnvVariable {
                key,
                value,
                updated_at: None,
            })
            .collect();

        let environment = Environment {
            id: env_response.project.clone(),
            name: env_response.environment,
            variables,
        };

        Ok(environment)
    }

    pub async fn update_variables(
        &self,
        token: &str,
        project_id: &str,
        env_name: &str,
        variables: Vec<EnvVariable>,
    ) -> Result<()> {
        let url = format!(
            "{}/api/v1/projects/{}/{}",
            self.base_url, project_id, env_name
        );

        let vars_map: std::collections::HashMap<String, String> =
            variables.into_iter().map(|v| (v.key, v.value)).collect();

        #[derive(Serialize)]
        struct PushRequest {
            variables: std::collections::HashMap<String, String>,
        }

        let response = self
            .client
            .post(&url)
            .bearer_auth(token)
            .json(&PushRequest {
                variables: vars_map,
            })
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to update variables: {}", response.status());
        }

        Ok(())
    }

    pub async fn create_project(
        &self,
        token: &str,
        workspace_id: &str,
        name: &str,
    ) -> Result<Project> {
        let url = format!(
            "{}/api/v1/workspaces/{}/projects",
            self.base_url, workspace_id
        );

        #[derive(Serialize)]
        struct CreateProjectRequest {
            name: String,
        }

        let response = self
            .client
            .post(&url)
            .bearer_auth(token)
            .json(&CreateProjectRequest {
                name: name.to_string(),
            })
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to create project: {}", response.status());
        }

        let project = response.json::<Project>().await?;
        Ok(project)
    }
}
