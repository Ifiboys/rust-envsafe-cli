#![allow(deprecated)]

use crate::api::{ApiClient, EnvVariable};
use crate::config::Config;
use crate::storage::{EnvStorage, SharedEnvData};
use anyhow::Result;
use chrono::Utc;
use colored::*;
use notify::event::{DataChange, ModifyKind};
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::Path;
use tokio::sync::mpsc;

pub struct EnvWatcher {
    api_client: ApiClient,
    config: Config,
    storage: EnvStorage,
}

impl EnvWatcher {
    pub fn new(api_client: ApiClient, config: Config) -> Result<Self> {
        let storage = EnvStorage::new()?;
        Ok(Self {
            api_client,
            config,
            storage,
        })
    }

    /// Watch for changes from remote (WebSocket) and update local shared memory
    pub async fn watch_remote(&mut self, project_id: &str, environment: &str) -> Result<()> {
        println!("{}", "🔄 Starting hot reload watcher...".cyan());
        println!("{}", format!("  Project: {}", project_id).bright_black());
        println!(
            "{}",
            format!("  Environment: {}", environment).bright_black()
        );
        println!("{}", "  Watching for remote changes...".bright_black());

        let token = self.config.get_token()?;
        let ws_url = if let Some(ws_url) = &self.config.ws_url {
            ws_url.trim_end_matches('/').to_string()
        } else {
            match std::env::var("ENVSAFE_WS_URL") {
                Ok(url) => url.trim_end_matches('/').to_string(),
                Err(_) => "wss://socket-server-production-79a0.up.railway.app".to_string(),
            }
        };
        let ws_url = format!(
            "{}/api/ws/projects/{}/environments/{}?token={}",
            ws_url, project_id, environment, token
        );

        // Connect to WebSocket
        let (ws_stream, _) = tokio_tungstenite::connect_async(&ws_url).await?;
        println!("{}", "✓ WebSocket connected".green());

        use futures_util::StreamExt;
        let (_, mut read) = ws_stream.split();

        let mut current_version = self.storage.get_version()?;

        // Initial fetch
        self.fetch_and_update(project_id, environment, &token, &mut current_version)
            .await?;

        // Listen for updates
        while let Some(msg) = read.next().await {
            match msg {
                Ok(msg) => {
                    if let Ok(text) = msg.to_text() {
                        if text == "update" {
                            println!("{}", "📥 Remote change detected, updating...".yellow());
                            self.fetch_and_update(
                                project_id,
                                environment,
                                &token,
                                &mut current_version,
                            )
                            .await?;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}", format!("WebSocket error: {}", e).red());
                    break;
                }
            }
        }

        Ok(())
    }

    /// Watch local .env file for changes and sync to remote
    #[allow(dead_code)]
    pub async fn watch_local(
        &mut self,
        project_id: &str,
        environment: &str,
        file_path: &str,
    ) -> Result<()> {
        println!("{}", "📂 Watching local file for changes...".cyan());
        println!("{}", format!("  File: {}", file_path).bright_black());

        let (tx, mut rx) = mpsc::channel(100);

        let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
            if let Ok(event) = res {
                if matches!(
                    event.kind,
                    EventKind::Modify(ModifyKind::Data(DataChange::Any))
                ) {
                    let _ = tx.blocking_send(event);
                }
            }
        })?;

        watcher.watch(Path::new(file_path), RecursiveMode::NonRecursive)?;

        println!("{}", "✓ File watcher started".green());

        let token = self.config.get_token()?;

        while let Some(_event) = rx.recv().await {
            println!(
                "{}",
                "📤 Local change detected, syncing to remote...".yellow()
            );

            // Read .env file
            match dotenv::from_path_iter(file_path) {
                Ok(iter) => {
                    let variables: Vec<EnvVariable> = iter
                        .filter_map(|item| item.ok())
                        .map(|(key, value)| EnvVariable {
                            key,
                            value,
                            updated_at: Some(Utc::now().to_rfc3339()),
                        })
                        .collect();

                    // Push to remote
                    match self
                        .api_client
                        .update_variables(&token, project_id, environment, variables.clone())
                        .await
                    {
                        Ok(_) => {
                            println!("{}", "✓ Successfully synced to remote".green());

                            // Update shared memory
                            let vars_map: HashMap<String, String> =
                                variables.into_iter().map(|v| (v.key, v.value)).collect();

                            let current_version = self.storage.get_version()?;
                            let data = SharedEnvData {
                                version: current_version + 1,
                                project_id: project_id.to_string(),
                                environment: environment.to_string(),
                                variables: vars_map,
                                last_updated: Utc::now().to_rfc3339(),
                            };

                            self.storage.write(&data)?;
                        }
                        Err(e) => {
                            eprintln!("{}", format!("Failed to sync: {}", e).red());
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}", format!("Failed to read .env file: {}", e).red());
                }
            }
        }

        Ok(())
    }

    async fn fetch_and_update(
        &mut self,
        project_id: &str,
        environment: &str,
        token: &str,
        current_version: &mut u64,
    ) -> Result<()> {
        let env = self
            .api_client
            .get_environment(token, project_id, environment)
            .await?;

        let vars_map: HashMap<String, String> = env
            .variables
            .iter()
            .map(|v| (v.key.clone(), v.value.clone()))
            .collect();

        *current_version += 1;

        let data = SharedEnvData {
            version: *current_version,
            project_id: project_id.to_string(),
            environment: environment.to_string(),
            variables: vars_map.clone(),
            last_updated: Utc::now().to_rfc3339(),
        };

        self.storage.write(&data)?;

        // Update local .env file
        self.update_local_file(project_id, environment, &vars_map)?;

        println!(
            "{}",
            format!(
                "✓ Updated {} variables (v{})",
                vars_map.len(),
                current_version
            )
            .green()
        );

        Ok(())
    }

    fn update_local_file(
        &self,
        project_id: &str,
        environment: &str,
        remote_vars: &HashMap<String, String>,
    ) -> Result<()> {
        let file_path = ".env";
        let path = Path::new(file_path);
        let mut local_only_vars: Vec<(String, String)> = Vec::new();

        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some((key, value)) = line.split_once('=') {
                     let key = key.trim().to_string();
                     let value = value.trim().to_string();
                     if !remote_vars.contains_key(&key) {
                         local_only_vars.push((key, value));
                     }
                }
            }
        }

        let mut content = String::new();
        content.push_str(&format!("# EnvSafe - {}\n", project_id));
        content.push_str(&format!("# Environment: {}\n", environment));
        content.push_str(&format!("# Updated: {}\n", Utc::now().to_rfc3339()));
        content.push_str("# Variables from EnvSafe (managed remotely)\n\n");

        // Write API variables (sorted)
        let mut sorted_keys: Vec<_> = remote_vars.keys().collect();
        sorted_keys.sort();
        for key in sorted_keys {
            if let Some(value) = remote_vars.get(key) {
                content.push_str(&format!("{}={}\n", key, value));
            }
        }

        // Write local-only variables
        if !local_only_vars.is_empty() {
            content.push_str("\n# Local variables (not managed by EnvSafe)\n");
            for (key, value) in local_only_vars {
                content.push_str(&format!("{}={}\n", key, value));
            }
        }

        std::fs::write(path, content)?;
        
        Ok(())
    }
}
