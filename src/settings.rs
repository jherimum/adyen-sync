use anyhow::{Context, Result};
use config::File;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir, OpenOptions},
    io::Write,
    path::PathBuf,
};

const CONFIG_FOLDER_NAME: &str = ".adyen-sync";
const CONFIG_FILE_NAME: &str = "config.json";

pub trait MergeSettings {
    fn merge(self, settings: &Settings) -> Self;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub source_url: Option<String>,
    pub target_url: Option<String>,
    pub timeout: Option<u64>,
    pub target_client_id: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            source_url: Default::default(),
            target_url: Default::default(),
            timeout: Some(10),
            target_client_id: Default::default(),
        }
    }
}

impl Settings {
    pub fn write(&self) -> Result<()> {
        let config_file_path = Self::config_file_path()?;
        let mut config_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_file_path)
            .context("Error while opening config file")?;

        let settings_json =
            serde_json::to_string_pretty(self).expect("Error while serializing settings");
        config_file
            .write_all(settings_json.as_bytes())
            .context("Error while writing to config file")?;
        config_file
            .flush()
            .context("Error while flushing config file")
    }

    pub fn load() -> Result<Self> {
        let config_file = Self::config_file_path()?;

        if !config_file.exists() {
            if let Some(file_dir) = config_file.parent() {
                if !file_dir.exists() {
                    create_dir(file_dir).context(format!(
                        "Error while creating config folder: {}",
                        file_dir.display()
                    ))?;
                }
            }

            Self::default().write()?
        }

        config::Config::builder()
            .add_source(config::Environment::with_prefix("ADYEN_SYNC"))
            .add_source(File::from(config_file))
            .build()
            .context("Erro while building configuration")?
            .try_deserialize::<Self>()
            .context("Erro while deserializing to settings")
    }

    pub fn config_file_path() -> Result<PathBuf> {
        Ok(home::home_dir()
            .context("User home could not be found")?
            .join(CONFIG_FOLDER_NAME)
            .join(CONFIG_FILE_NAME))
    }

    pub fn target_url(&mut self, url: &Option<String>) {
        if let Some(url) = url {
            self.target_url = Some(url.clone())
        }
    }

    pub fn source_url(&mut self, url: &Option<String>) {
        if let Some(url) = url {
            self.source_url = Some(url.clone())
        }
    }

    pub fn timeout(&mut self, timeout: &Option<u64>) {
        if let Some(timeout) = timeout {
            self.timeout = Some(*timeout)
        }
    }

    pub fn target_client_id(&mut self, target_client_id: &Option<String>) {
        if let Some(target_client_id) = target_client_id {
            self.target_client_id = Some(target_client_id.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_globals() {}
}
