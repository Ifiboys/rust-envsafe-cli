#![allow(dead_code)]

use anyhow::Result;
use std::collections::HashMap;

pub fn parse_env_file(content: &str) -> Result<HashMap<String, String>> {
    let mut vars = HashMap::new();

    for line in content.lines() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse KEY=VALUE
        if let Some(pos) = line.find('=') {
            let key = line[..pos].trim().to_string();
            let value = line[pos + 1..].trim();

            // Remove quotes if present
            let value = if (value.starts_with('"') && value.ends_with('"'))
                || (value.starts_with('\'') && value.ends_with('\''))
            {
                &value[1..value.len() - 1]
            } else {
                value
            };

            vars.insert(key, value.to_string());
        }
    }

    Ok(vars)
}

pub fn format_env_file(vars: &HashMap<String, String>) -> String {
    let mut lines: Vec<String> = vars
        .iter()
        .map(|(k, v)| {
            // Quote value if it contains spaces
            if v.contains(' ') {
                format!("{}=\"{}\"", k, v)
            } else {
                format!("{}={}", k, v)
            }
        })
        .collect();

    lines.sort();
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_env_file() {
        let content = r#"
# Comment
DATABASE_URL=postgres://localhost/db
API_KEY="secret key with spaces"
DEBUG=true
        "#;

        let vars = parse_env_file(content).unwrap();

        assert_eq!(vars.get("DATABASE_URL").unwrap(), "postgres://localhost/db");
        assert_eq!(vars.get("API_KEY").unwrap(), "secret key with spaces");
        assert_eq!(vars.get("DEBUG").unwrap(), "true");
    }

    #[test]
    fn test_format_env_file() {
        let mut vars = HashMap::new();
        vars.insert(
            "DATABASE_URL".to_string(),
            "postgres://localhost/db".to_string(),
        );
        vars.insert("API_KEY".to_string(), "secret key".to_string());

        let formatted = format_env_file(&vars);

        assert!(formatted.contains("DATABASE_URL=postgres://localhost/db"));
        assert!(formatted.contains("API_KEY=\"secret key\""));
    }
}
