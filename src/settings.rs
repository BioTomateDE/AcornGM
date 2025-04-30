use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcornSettings {
    pub access_token: Option<String>,   // your AcornGM access token
}

pub fn load_settings(home_dir: &PathBuf, is_first_launch: bool) -> Result<AcornSettings, String> {
    let path: PathBuf = home_dir.join("settings.json");
    if is_first_launch && !path.is_file() {
        save_settings(home_dir, &Default::default())?;
        return Ok(Default::default())   // return default settings on first launch instead of error message
    }

    let string: String = fs::read_to_string(&path)
        .map_err(|e| format!("Could not read settings file at {path:?}: {e}"))?;

    let settings: AcornSettings = serde_json::from_str::<AcornSettings>(&string)
        .map_err(|e| format!("Could not parse settings json: {e}"))?;

    Ok(settings)
}

pub fn save_settings(home_dir: &PathBuf, settings: &AcornSettings) -> Result<(), String> {
    let path: PathBuf = home_dir.join("settings.json");

    let string: String = serde_json::to_string(settings)
        .map_err(|e| format!("Could not build settings json: {e}"))?;

    fs::write(&path, string)
        .map_err(|e| format!("Could not write settings file at {path:?}: {e}"))?;

    Ok(())
}

