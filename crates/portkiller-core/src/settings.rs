use std::fs;
use std::path::PathBuf;

use crate::models::{AppError, AppSettings};

pub struct Store {
    path: PathBuf,
}

impl Store {
    pub fn new() -> Result<Self, AppError> {
        let dir = settings_dir()?;
        fs::create_dir_all(&dir)?;
        Ok(Self {
            path: dir.join("config.json"),
        })
    }

    pub fn load(&self) -> Result<AppSettings, AppError> {
        if !self.path.exists() {
            let defaults = AppSettings::default();
            self.save(&defaults)?;
            return Ok(defaults);
        }
        let data = fs::read_to_string(&self.path)?;
        serde_json::from_str(&data)
            .map_err(|e| AppError::Settings(format!("invalid config: {e}")))
    }

    pub fn save(&self, settings: &AppSettings) -> Result<(), AppError> {
        let data = serde_json::to_string_pretty(settings)
            .map_err(|e| AppError::Settings(e.to_string()))?;
        fs::write(&self.path, data)?;
        Ok(())
    }

    pub fn toggle_favorite(&self, port: u16) -> Result<Vec<u16>, AppError> {
        let mut settings = self.load()?;
        if let Some(idx) = settings.favorites.iter().position(|p| *p == port) {
            settings.favorites.remove(idx);
        } else {
            settings.favorites.push(port);
            settings.favorites.sort_unstable();
        }
        self.save(&settings)?;
        Ok(settings.favorites.clone())
    }

    pub fn add_favorite(&self, port: u16) -> Result<Vec<u16>, AppError> {
        let mut settings = self.load()?;
        if !settings.favorites.contains(&port) {
            settings.favorites.push(port);
            settings.favorites.sort_unstable();
            self.save(&settings)?;
        }
        Ok(settings.favorites.clone())
    }

    pub fn remove_favorite(&self, port: u16) -> Result<Vec<u16>, AppError> {
        let mut settings = self.load()?;
        settings.favorites.retain(|p| *p != port);
        self.save(&settings)?;
        Ok(settings.favorites.clone())
    }

    pub fn config_path(&self) -> &PathBuf {
        &self.path
    }
}

pub fn settings_dir() -> Result<PathBuf, AppError> {
    let base = dirs::data_dir().ok_or_else(|| AppError::Settings("no app data dir".into()))?;
    Ok(base.join("portkiller"))
}
