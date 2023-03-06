use anyhow::{anyhow, Context, Result};
use config::File;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir, OpenOptions},
    io::Write,
    path::PathBuf,
};

const CONFIG_FOLDER_NAME: &str = ".adyen-sync";
const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Settings {
    pub source_url: Option<String>,
    pub target_url: Option<String>,
}

impl Settings {
    pub fn write(&self) -> Result<()> {
        let config_file_path = Self::config_file_path()?;
        let mut config_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_file_path)?;

        let settings_json = serde_json::to_string_pretty(self).expect("json");
        config_file.write_all(settings_json.as_bytes())?;
        config_file.flush().context("context")
    }

    pub fn load() -> Result<Self> {
        let config_file = Self::config_file_path()?;

        if !config_file.exists() {
            if let Some(file_dir) = config_file.parent() {
                if !file_dir.exists() {
                    create_dir(file_dir)?;
                }
            }

            Self::default().write()?
        }

        config::Config::builder()
            .add_source(config::Environment::with_prefix("ADYEN_SYNC"))
            .add_source(File::from(config_file))
            .build()?
            .try_deserialize::<Self>()
            .context("context")
    }

    fn config_file_path() -> Result<PathBuf> {
        home::home_dir()
            .map(|h| h.join(CONFIG_FOLDER_NAME).join(CONFIG_FILE_NAME))
            .ok_or_else(|| anyhow!(""))
    }

    pub fn update_target_url(&mut self, url: Option<String>) {
        if let Some(url) = url {
            self.target_url = Some(url)
        }
    }

    pub fn update_source_url(&mut self, url: Option<String>) {
        if let Some(url) = url {
            self.source_url = Some(url)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_globals() {}
}
