use std::fmt;
use std::path::PathBuf;
use iced::Color;
use iced::widget::button;
use iced::widget::image::Handle;

pub fn get_default_icon_image(cwd: &PathBuf) -> Handle {
    let path: PathBuf = std::path::Path::new(&cwd).join("./resources/textures/default_profile_icon.png");
    if !path.is_file() {
        println!("[WARN @ utility::get_default_icon_image]  Could not get default icon because its path doesn't exist: {}", path_to_str(&path));
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


pub fn path_to_str(path: &PathBuf) -> &str {
    path.to_str().unwrap_or_else(|| "<invalid os string>")
}


pub static BASE_URL: &'static str = "https://acorngmbackend.onrender.com/";


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

        Ok(Version { major, minor })
    }
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