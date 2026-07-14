use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortInfo {
    pub port: u16,
    pub pid: u32,
    pub process_name: String,
    pub address: String,
    pub user: String,
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppSettings {
    #[serde(default = "default_favorites")]
    pub favorites: Vec<u16>,
    #[serde(default = "default_refresh")]
    pub refresh_interval_secs: u64,
    #[serde(default)]
    pub start_minimized: bool,
    #[serde(default)]
    pub autostart: bool,
    #[serde(default = "default_true")]
    pub close_to_tray: bool,
    #[serde(default = "default_true")]
    pub monitoring_enabled: bool,
    #[serde(default = "default_locale")]
    pub locale: String,
    #[serde(default = "default_true")]
    pub auto_check_updates: bool,
}

fn default_favorites() -> Vec<u16> {
    vec![3000, 5173, 8080]
}

fn default_refresh() -> u64 {
    5
}

fn default_true() -> bool {
    true
}

fn default_locale() -> String {
    "en".to_string()
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            favorites: default_favorites(),
            refresh_interval_secs: default_refresh(),
            start_minimized: false,
            autostart: false,
            close_to_tray: true,
            monitoring_enabled: true,
            locale: default_locale(),
            auto_check_updates: true,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("settings error: {0}")]
    Settings(String),
    #[error("port not found: {0}")]
    PortNotFound(u16),
    #[error("process not found: {0}")]
    ProcessNotFound(u32),
    #[error("access denied")]
    AccessDenied,
    #[error("windows api error: {0}")]
    WindowsApi(String),
    #[error("{0}")]
    Other(String),
}

impl AppError {
    pub fn exit_code(&self) -> i32 {
        match self {
            AppError::PortNotFound(_) | AppError::ProcessNotFound(_) => 2,
            AppError::AccessDenied => 3,
            _ => 1,
        }
    }
}
