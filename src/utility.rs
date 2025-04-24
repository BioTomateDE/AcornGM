use std::{fmt, fs};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use iced::Color;
use iced::widget::button;
use iced::widget::image::Handle;
use log::{error, info};
use serde::Serialize;
use sha2::{Digest, Sha256};
use crate::default_file_paths::get_resource_image_path;

pub fn get_default_icon_image(app_root: &PathBuf) -> Handle {
    let path: PathBuf = get_resource_image_path(app_root, "default_profile_icon.png");
    if !path.is_file() {
        error!("Could not get default icon because its path doesn't exist: {}", path.display());
        return Handle::from_pixels(1, 1, [0, 0, 0, 0])
    }

    Handle::from_path(path)
}


#[derive(Default, Debug, Clone)]
pub enum GameType {
    #[default]
    Unset,
    Undertale,
    Deltarune,
    Other(String),
}
impl GameType {
    pub fn from_name(name: &str) -> Self {
        match name {
            "Undertale" => Self::Undertale,
            "Deltarune" => Self::Deltarune,
            other => Self::Other(other.to_string()),
        }
    }
    pub fn to_string(&self) -> Option<String> {
        match &self {
            GameType::Unset => None,
            GameType::Undertale => Some("Undertale".to_string()),
            GameType::Deltarune => Some("Deltarune".to_string()),
            GameType::Other(name) => Some(name.clone()),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct GameInfo {
    pub game_type: GameType,
    pub version: Version,
}



#[derive(Debug, Clone, Copy)]
pub struct TransparentButton;
impl button::StyleSheet for TransparentButton {
    type Style = iced::Theme;
    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: Default::default(),
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            border: Default::default(),
            shadow: Default::default(),
            shadow_offset: iced::Vector::default(),
        }
    }
}

// pub const ACORN_BASE_URL: &'static str = "http://localhost:8000";
pub const ACORN_BASE_URL: &'static str = "https://acorngm.onrender.com";

#[derive(Debug)]
pub struct ParseVersionError;
impl std::error::Error for ParseVersionError {}
impl fmt::Display for ParseVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid version format")
    }
}


#[derive(Debug, Clone, Default)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
}

impl Version {
    pub fn from_vec(vec: [u32; 2]) -> Self {
        Self {
            major: vec[0],
            minor: vec[1],
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:02}", self.major, self.minor)
    }
}

impl std::str::FromStr for Version {
    type Err = ParseVersionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');
        let major = parts
            .next()
            .ok_or(ParseVersionError)?
            .parse()
            .map_err(|_| ParseVersionError)?;
        let minor = parts
            .next()
            .ok_or(ParseVersionError)?
            .parse()
            .map_err(|_| ParseVersionError)?;

        // Ensure there are no extra parts after the minor version
        if parts.next().is_some() {
            return Err(ParseVersionError);
        }

        Ok(Version { major, minor })
    }
}

pub fn remove_spaces(s: &str) -> String {
    s.chars().filter(|&c| !c.is_whitespace()).collect()
}

pub fn hash_file(path: &Path) -> Result<String, String> {
    let file = fs::File::open(path)
        .map_err(|e| format!("[ERROR @ utility::hash_file2]  Could not open data file '{}': {}", path.display(), e))?;
    let mut reader = BufReader::new(file);

    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)
            .map_err(|e| format!("[ERROR @ utility::hash_file2]  Could not read from data file '{}': {}", path.display(), e))?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}


#[derive(Default, Debug, Clone)]
pub enum PlatformType {
    #[default]
    Unset,
    Linux,
    Windows,
    MacOS,
    Android,
    IOS,
    // Other(String),
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub distro_pretty: String,
    pub platform_pretty: String,
    pub desktop_environment_pretty: String,
    pub cpu_architecture: String,
}

pub fn get_device_info() -> DeviceInfo {
    DeviceInfo {
        distro_pretty: whoami::fallible::distro().unwrap_or_else(|_| "<unknown distro>".to_string()),
        platform_pretty: whoami::platform().to_string(),
        desktop_environment_pretty: whoami::desktop_env().to_string(),
        cpu_architecture: whoami::arch().to_string(),
    }
}


pub fn show_error_dialogue(title: &str, message: &str) {
    info!("Showing Message Dialogue: {message}");
    let message_dialogue: rfd::MessageDialog = rfd::MessageDialog::new()
        .set_title(title)
        .set_description(message)
        .set_buttons(rfd::MessageButtons::Ok)
        .set_level(rfd::MessageLevel::Error);

    std::thread::spawn(|| message_dialogue.show());
}

