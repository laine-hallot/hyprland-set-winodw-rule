use std::path::PathBuf;

use color_eyre::eyre::{self, WrapErr};
use directories::{ProjectDirs, UserDirs};

pub fn get_data_dir() -> eyre::Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var("RATATUI_TEMPLATE_DATA") {
        PathBuf::from(s)
    } else if let Some(proj_dirs) = ProjectDirs::from("com", "kdheepak", "ratatui-template") {
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        return Err(eyre::eyre!(
            "Unable to find data directory for ratatui-template"
        ));
    };
    Ok(directory)
}

pub fn get_config_dir() -> eyre::Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var("RATATUI_TEMPLATE_CONFIG") {
        PathBuf::from(s)
    } else if let Some(proj_dirs) = ProjectDirs::from("com", "kdheepak", "ratatui-template") {
        proj_dirs.config_local_dir().to_path_buf()
    } else {
        return Err(eyre::eyre!(
            "Unable to find config directory for ratatui-template"
        ));
    };
    Ok(directory)
}

pub fn get_hyprland_dir() -> eyre::Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var("HYPRLAND_CONFIG_DIR") {
        PathBuf::from(s)
    } else if let Some(home_dir) = UserDirs::new() {
        home_dir.home_dir().join(".config/hypr/")
    } else {
        return Err(eyre::eyre!(
            "Unable to find data directory for ratatui-template"
        ));
    };
    Ok(directory)
}
