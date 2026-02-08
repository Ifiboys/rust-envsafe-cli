#![allow(dead_code)]

use anyhow::Result;
use serde::{Deserialize, Serialize};
use shared_memory::*;
use std::collections::HashMap;

const SHMEM_SIZE: usize = 1024 * 1024; // 1MB
const SHMEM_NAME: &str = "envsafe_env_vars";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedEnvData {
    pub version: u64,
    pub project_id: String,
    pub environment: String,
    pub variables: HashMap<String, String>,
    pub last_updated: String,
}

pub struct EnvStorage {
    shmem: Shmem,
}

impl EnvStorage {
    /// Create or open shared memory
    pub fn new() -> Result<Self> {
        let shmem = match ShmemConf::new().size(SHMEM_SIZE).os_id(SHMEM_NAME).create() {
            Ok(m) => m,
            Err(ShmemError::LinkExists) => {
                // Memory already exists, open it
                ShmemConf::new().os_id(SHMEM_NAME).open()?
            }
            Err(e) => return Err(e.into()),
        };

        Ok(Self { shmem })
    }

    /// Write environment variables to shared memory
    pub fn write(&mut self, data: &SharedEnvData) -> Result<()> {
        let json = serde_json::to_vec(data)?;

        if json.len() > SHMEM_SIZE - 8 {
            anyhow::bail!("Data too large for shared memory");
        }

        unsafe {
            let ptr = self.shmem.as_ptr();

            // Write length as first 8 bytes
            let len_bytes = (json.len() as u64).to_le_bytes();
            std::ptr::copy_nonoverlapping(len_bytes.as_ptr(), ptr, 8);

            // Write data
            std::ptr::copy_nonoverlapping(json.as_ptr(), ptr.add(8), json.len());
        }

        Ok(())
    }

    /// Read environment variables from shared memory
    pub fn read(&self) -> Result<Option<SharedEnvData>> {
        unsafe {
            let ptr = self.shmem.as_ptr() as *const u8;

            // Read length
            let mut len_bytes = [0u8; 8];
            std::ptr::copy_nonoverlapping(ptr, len_bytes.as_mut_ptr(), 8);
            let len = u64::from_le_bytes(len_bytes) as usize;

            if len == 0 || len > SHMEM_SIZE - 8 {
                return Ok(None);
            }

            // Read data
            let mut buffer = vec![0u8; len];
            std::ptr::copy_nonoverlapping(ptr.add(8), buffer.as_mut_ptr(), len);

            let data: SharedEnvData = serde_json::from_slice(&buffer)?;
            Ok(Some(data))
        }
    }

    /// Clear shared memory
    pub fn clear(&mut self) -> Result<()> {
        unsafe {
            let ptr = self.shmem.as_ptr();
            std::ptr::write_bytes(ptr, 0, SHMEM_SIZE);
        }
        Ok(())
    }

    /// Get current version
    pub fn get_version(&self) -> Result<u64> {
        match self.read()? {
            Some(data) => Ok(data.version),
            None => Ok(0),
        }
    }
}

impl Drop for EnvStorage {
    fn drop(&mut self) {
        // Don't clear on drop, other processes might be using it
    }
}

/// Utility functions for accessing env vars from shared memory
pub mod access {
    use super::*;

    /// Get all environment variables from shared memory
    pub fn get_env_vars() -> Result<HashMap<String, String>> {
        let storage = EnvStorage::new()?;
        match storage.read()? {
            Some(data) => Ok(data.variables),
            None => Ok(HashMap::new()),
        }
    }

    /// Get a specific environment variable
    pub fn get_env(key: &str) -> Result<Option<String>> {
        let vars = get_env_vars()?;
        Ok(vars.get(key).cloned())
    }

    /// Check if shared memory has been updated
    pub fn check_version() -> Result<u64> {
        let storage = EnvStorage::new()?;
        storage.get_version()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write() {
        let mut storage = EnvStorage::new().unwrap();

        let mut vars = HashMap::new();
        vars.insert("TEST_KEY".to_string(), "test_value".to_string());

        let data = SharedEnvData {
            version: 1,
            project_id: "test-project".to_string(),
            environment: "development".to_string(),
            variables: vars.clone(),
            last_updated: chrono::Utc::now().to_rfc3339(),
        };

        storage.write(&data).unwrap();

        let read_data = storage.read().unwrap().unwrap();
        assert_eq!(read_data.version, 1);
        assert_eq!(read_data.variables.get("TEST_KEY").unwrap(), "test_value");

        storage.clear().unwrap();
    }
}
