use std::{fmt, fs};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use iced::Color;
use iced::widget::button;
use iced::widget::image::Handle;
use serde::Serialize;
use sha2::{Digest, Sha256};

pub fn get_default_icon_image(cwd: &PathBuf) -> Handle {
    let path: PathBuf = std::path::Path::new(&cwd).join("./resources/textures/default_profile_icon.png");
    if !path.is_file() {
        println!("[WARN @ utility::get_default_icon_image]  Could not get default icon because its path doesn't exist: {}", path.display());
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
pub struct DeviceInfo {
    pub host_name: String,
    pub distro_pretty: String,
    pub platform_pretty: String,
    pub desktop_environment_pretty: String,
    pub cpu_architecture: String,
}

pub fn get_device_info() -> DeviceInfo {
    DeviceInfo {
        host_name: whoami::fallible::hostname().unwrap_or_else(|_| "<unknown hostname>".to_string()),
        distro_pretty: whoami::fallible::distro().unwrap_or_else(|_| "<unknown distro>".to_string()),
        platform_pretty: whoami::platform().to_string(),
        desktop_environment_pretty: whoami::desktop_env().to_string(),
        cpu_architecture: whoami::arch().to_string(),
    }
}