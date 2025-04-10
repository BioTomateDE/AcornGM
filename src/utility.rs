use std::fmt;
use std::path::PathBuf;
use iced::Color;
use iced::widget::button;
use iced::widget::image::Handle;
use crate::default_file_paths::show_msgbox;

pub fn get_current_working_directory() -> Option<String> {
    match std::env::current_dir() {
        Ok(path) => match path.to_str() {
            Some(string) => Some(string.to_string()),
            None => {
                println!("[WARN]  Could not parse string of current working directory");
                None
            }
        },
        Err(error ) => {
            println!("[WARN]  Could not get current working directory: {error}");
            None
        }
    }
}


fn try_get_default_icon_image() -> Option<image::DynamicImage> {
    let cwd: String = match get_current_working_directory() {
        Some(cwd) => cwd,
        None => return None,
    };
    let path: PathBuf = std::path::Path::new(&cwd).join("./resources/textures/default_profile_icon.png");

    let img: image::DynamicImage = match image::open(path) {
        Ok(raw) => raw,
        Err(error) => {
            println!("[WARN @ utility::try_get_default_icon_image]  Failed to read default icon image: {error}");
            return None
        }
    };

    Some(img)
}


// pub fn get_local_font(font_filename: &str) -> Result<Command<Result<(), !>>, String> {
//     let cwd: String = match get_current_working_directory() {
//         Some(cwd) => cwd,
//         None => return Err("Could not get current working directory while getting fonts!".to_string()),
//     };
//     let path: PathBuf = std::path::Path::new(&cwd).join(format!("./resources/fonts/{font_filename}"));
//     let raw = match fs::read(&path) {
//         Ok(bytes) => bytes,
//         Err(error) => return Err(format!("Could not read font file \"{}\": {error}", path.to_str().unwrap_or_else(|| "<could not convert path to string>"))),
//     };
//
//     let command = iced::font::load(raw);
//     Ok(command)
// }

pub fn get_default_icon_image() -> Handle {
    let img = try_get_default_icon_image().unwrap_or_else(
        || image::DynamicImage::ImageRgba8(image::RgbaImage::new(256, 256)));
    img_to_iced(&img)
}


pub fn img_to_iced(img: &image::DynamicImage) -> Handle {
    let mut buf = std::io::Cursor::new(Vec::new());
    if img.write_to(&mut buf, image::ImageOutputFormat::Png).is_err() {
        show_msgbox("Error while converting image", "Could not write DynamicImage to buffer.");
        return Handle::from_pixels(1, 1, [0, 0, 0, 0])
    };
    Handle::from_memory(buf.into_inner())
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