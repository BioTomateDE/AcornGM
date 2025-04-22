mod create_profile1;
pub use create_profile1::MsgCreateProfile1;
mod create_profile2;
pub use create_profile2::MsgCreateProfile2;

use std::path::{Path, PathBuf};
use fast_image_resize::PixelType;
use fast_image_resize as fir;
use iced::{Command, Element};
use iced::advanced::image::Data;
use iced::widget::image::Handle;
use iced::widget::text;
use image::DynamicImage;
use log::error;
use crate::{Msg, MyApp, Scene};
use crate::utility::{hash_file, GameInfo, GameType, Version};


#[derive(Debug, Clone)]
pub struct SceneCreateProfile {
    pub stage: u8,
    pub profile_name: String,
    pub is_profile_name_valid: bool,
    pub icon: Handle,
    pub data_file_path: String,
    pub game_info: GameInfo,
    pub game_name: String,      // used as a buffer for text input; represents .game_info(GameInfo::Other(string))
    pub game_version_str: String,
    pub is_game_version_valid: bool,
}

impl Scene for SceneCreateProfile {
    fn update(&mut self, app: &mut MyApp, message: Msg) -> Command<Msg> {
        match self.stage {
            1 => self.update1(app, message),
            2 => self.update2(app, message),
            other => {
                error!("Invalid scene stage {other}");
                Command::none()
            }
        }
    }
    fn view(&self, app: &MyApp) -> Element<Msg> {
        match &self.stage {
            1 => self.view1(app),
            2 => self.view2(app),
            other => {
                error!("Invalid scene stage {other}");
                text("Error").into()
            }
        }
    }
}


fn check_profile_name_valid(profile_name: &str) -> bool {
    let profile_name: &str = profile_name.trim();

    profile_name.len() < 100 &&
        profile_name.len() > 0
}

fn make_profile_dir_name_valid(profile_name: &str) -> String {
    static BANNED_CHARS: [char; 15] = ['.', '/', '\\', '\n', '\r', '\t', '<', '>', ':', '"', '\'', '|', '?', '*', ' '];
    static BANNED_NAMES: [&'static str; 22] = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
        "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"];

    let mut name: String = String::with_capacity(profile_name.len());

    for char in profile_name.chars() {
        if !BANNED_CHARS.contains(&char) {
            name.push(char);
        }
    }

    // fallback name for directory
    if name.len() < 1 || name.ends_with('.') || BANNED_NAMES.contains(&name.to_uppercase().as_str()) {
        name = uuid::Uuid::new_v4().hyphenated().to_string();
    }
    name
}

fn detect_game_and_version(data_file_path: &Path) -> Result<GameInfo, String> {
    let hash: String = hash_file(data_file_path)?;      // {..} SLOW OPERATION
    println!("Game data.win SHA-256 Hash: {hash}");

    match hash.as_str() {
        "7f3e3d6ddc5e6ba3bd45f94c1d6277becbbf3a519d1941d321289d7d2b9f5d26" => Ok(GameInfo {
            game_type: GameType::Undertale,
            version: Version {major: 1, minor: 0},
        }),
        "e59b57224b33673c4d1a33d99bcb24fe915061ea3f223d652aaf159d00cbfca8" |
        "3f85bc6204c2bf4975515e0f5283f5256e2875c81d8746db421182abd7123b08" => Ok(GameInfo {
            game_type: GameType::Undertale,
            version: Version {major: 1, minor: 1},
        }),
        "8804cabdcd91777b07f071955e4189384766203ae72d6fbaf828e1ab0948c856" => Ok(GameInfo {
            game_type: GameType::Undertale,
            version: Version {major: 1, minor: 6},
        }),
        "cd6dfa453ce9f1122cbd764921f9baa7f4289470717998a852b8f6ca8d6bb334" |
        "b718f8223a5bb31979ffeed10be6140c857b882fc0d0462b89d6287ae38c81c7" => Ok(GameInfo {
            game_type: GameType::Undertale,
            version: Version {major: 1, minor: 8},
        }),
        "c346f0a0a1ba02ac2d2db84df5dbf31d5ae28c64d8b65b8db6af70c67c430f39" |
        "4de4118ba4ad4243025e61337fe796532751032c0a04d0843d8b35f91ec2c220" |
        "45e594c06dfc48c14a2918efe7eb1874876c47b23b232550f910ce0e52de540d" => Ok(GameInfo {
            game_type: GameType::Deltarune,
            version: Version {major: 2, minor: 0},
        }),
        _ => Ok(GameInfo {
            game_type: GameType::Other("Other Game".to_string()),
            version: Version {major: 0, minor: 0},
        })
    }
}


fn resize_and_save_icon(handle: &Handle, path: PathBuf) -> Result<(), String> {
    const RESIZE_WIDTH: u32 = 256;
    const RESIZE_HEIGHT: u32 = 256;

    let fir_image_original: fir::images::Image = match handle.data() {
        Data::Path(path) =>
            convert_dynamic_image_to_fir(image::open(path)
                .map_err(|e| format!("Could not load icon image from path: {e}"))?
            )?,

        Data::Bytes(bytes) =>
            convert_dynamic_image_to_fir(image::load_from_memory(&bytes)
                .map_err(|e| format!("Could not load icon image from raw bytes: {e}"))?,
            )?,

        Data::Rgba { width, height, pixels } =>
            fir::images::Image::from_vec_u8(*width, *height, pixels.to_vec(), PixelType::U8x4)
                .map_err(|e| format!("Could not load icon image from in-memory RGBA: {e}"))?
    };

    let mut fir_img_resized = fir::images::Image::new(RESIZE_WIDTH, RESIZE_HEIGHT, PixelType::U8x4);
    fir::Resizer::new().resize(&fir_image_original, &mut fir_img_resized, None)
        .map_err(|e| format!("Could not resize icon image: {e}"))?;

    let img_resized: image::RgbaImage = image::ImageBuffer::from_raw(RESIZE_WIDTH, RESIZE_HEIGHT, fir_img_resized.into_vec())
        .ok_or("Could not convert fast_image_resize::images::Image to DynamicImage")?;

    img_resized.save(path)
        .map_err(|e| format!("Could not save icon image file: {e}"))?;

    Ok(())
}

fn convert_dynamic_image_to_fir(image: DynamicImage) -> Result<fir::images::Image<'static>, String> {
    let raw: Vec<u8> = image.to_rgba8().into_raw();
    Ok(fir::images::Image::from_vec_u8(image.width(), image.height(), raw, PixelType::U8x4)
        .map_err(|e| format!("Could not convert DynamicImage to fast_image_resize::images::Image: {e}"))?
    )
}

