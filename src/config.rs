use anyhow::Result;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub token: Option<String>,
    pub api_url: String,
    pub ws_url: Option<String>,
    pub dashboard_url: String,
    pub language: String,
    pub current_workspace: Option<String>,
    #[serde(default)]
    pub current_workspace_slug: Option<String>,
    pub current_project: Option<String>,
    #[serde(default)]
    pub current_project_slug: Option<String>,
    pub rotation: RotationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationConfig {
    pub enabled: bool,
    pub interval_days: u32,
    pub exclude_vars: Vec<String>,
    pub last_rotation: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            token: None,
            api_url: "https://www.envsafe.dev".to_string(),
            ws_url: Some("wss://socket-server-production-79a0.up.railway.app".to_string()),
            dashboard_url: "https://www.envsafe.dev".to_string(),
            language: "en".to_string(),
            current_workspace: None,
            current_workspace_slug: None,
            current_project: None,
            current_project_slug: None,
            rotation: RotationConfig {
                enabled: false,
                interval_days: 30,
                exclude_vars: vec![],
                last_rotation: None,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }

        let content = fs::read_to_string(&config_path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let config_dir =
            dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;

        Ok(config_dir.join("envsafe-cli").join("config.json"))
    }

    pub fn get_token(&self) -> Result<String> {
        self.token
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Not logged in. Please run: envsafe login"))
    }

    pub fn set_token(&mut self, token: String) -> Result<()> {
        self.token = Some(token);
        self.save()
    }

    pub fn clear_token(&mut self) -> Result<()> {
        self.token = None;
        self.save()
    }

    pub fn set_language(&mut self, lang: &str) -> Result<()> {
        self.language = lang.to_string();
        self.save()
    }

    pub fn set_workspace(
        &mut self,
        workspace_id: &str,
        workspace_slug: Option<&str>,
    ) -> Result<()> {
        self.current_workspace = Some(workspace_id.to_string());
        self.current_workspace_slug = workspace_slug.map(|s| s.to_string());
        self.save()
    }

    #[allow(dead_code)]
    pub fn set_project(&mut self, project_id: &str, project_slug: Option<&str>) -> Result<()> {
        self.current_project = Some(project_id.to_string());
        self.current_project_slug = project_slug.map(|s| s.to_string());
        self.save()
    }

    pub fn get_workspace_slug(&self) -> Result<String> {
        self.current_workspace_slug
            .clone()
            .or_else(|| self.current_workspace.clone())
            .ok_or_else(|| anyhow::anyhow!("No workspace linked. Please run: envsafe link"))
    }
}

/// Project configuration stored in local directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub workspace_id: String,
    pub project_id: String,
    pub project_name: String,
    #[serde(default)]
    pub project_slug: Option<String>,
}

impl ProjectConfig {
    pub fn load() -> Result<Option<Self>> {
        let config_path = PathBuf::from(".envsafe");

        if !config_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&config_path)?;
        let config: ProjectConfig = serde_json::from_str(&content)?;
        Ok(Some(config))
    }

    pub fn save(&self) -> Result<()> {
        let config_path = PathBuf::from(".envsafe");
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }
}
