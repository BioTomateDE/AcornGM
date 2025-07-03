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
use image::DynamicImage;
use log::info;
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
    pub is_file_picker_open: bool,
}

impl Scene for SceneCreateProfile {
    fn update(&mut self, app: &mut MyApp, message: Msg) -> Result<Command<Msg>, String> {
        match self.stage {
            1 => self.update1(app, message),
            2 => self.update2(app, message),
            other => Err(format!("Invalid scene stage {other}")),
        }
    }
    fn view(&self, app: &MyApp) -> Result<Element<Msg>, String> {
        match &self.stage {
            1 => self.view1(app),
            2 => self.view2(app),
            other => {
                Err(format!("Invalid scene stage {other}"))
            }
        }
    }
}


fn check_profile_name_valid(profile_name: &str) -> bool {
    let profile_name: &str = profile_name.trim();

    profile_name.len() < 100 &&
        profile_name.len() > 0
}

fn sanitize_profile_dir_name(profile_name: &str) -> String {
    static BANNED_CHARS: [char; 15] = ['.', '/', '\\', '\n', '\r', '\t', '<', '>', ':', '"', '\'', '|', '?', '*', ' '];
    static BANNED_NAMES: [&str; 22] = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
        "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"];

    let mut name: String = profile_name
        .chars()
        .filter(|c| !BANNED_CHARS.contains(c))
        .collect();

    // Truncate if too long (255 bytes is a common filesystem limit)
    if name.len() > 255 {
        name.truncate(255);
    }

    // Check for empty, trailing dot, or reserved names
    if name.is_empty() || name.ends_with('.') || BANNED_NAMES.iter().any(|&n| name.eq_ignore_ascii_case(n)) {
        uuid::Uuid::new_v4().hyphenated().to_string()
    } else {
        name
    }
}

fn detect_game_and_version(data_file_path: &Path) -> Result<GameInfo, String> {
    let hash: String = hash_file(data_file_path)?;      // {..} SLOW OPERATION
    info!("Game data.win Blake Hash: {hash}");

    match hash.as_str() {
        "baaf8d9e126834f5c323d73a2830d9ea47f960c3cea8bd569492f5c0758b8743" => Ok(GameInfo {
           game_type: GameType::Undertale,
            version: Version { major: 1, minor: 0 },
        }),
        "0fabda67409647e967a4604c2a1a0c6a310cd7d25b77236cb73f89dd723a8c3f" |
        "a0f3c642c45c0101eb4c2c4d821e2faf7bfadbcdaad9f94a766bda0b3e8af508" => Ok(GameInfo {
            game_type: GameType::Undertale,
            version: Version {major: 1, minor: 1},
        }),
        "08168e1cd456275d1a6bfe6076f2ac6ddfebdb2bfaae40d9e7f3d716b08bf10b" => Ok(GameInfo {
            game_type: GameType::Undertale,
            version: Version {major: 1, minor: 6},
        }),
        "14d6229a6760566c09e2a3465a4e363a5c4ed7ae87be8070a3e8eabd39c26e74" |
        "bc2358456dc4e55869fbe0cb89a41202fb4a00c13149fad4ef659ab6afe07025" => Ok(GameInfo {
            game_type: GameType::Undertale,
            version: Version {major: 1, minor: 8},
        }),
        "7d8535cc4232037e6c8a2c43e4739df79c8d7bf0c4cc32b149b04f93096d7a25" => Ok(GameInfo {
            game_type: GameType::Deltarune,
            version: Version {major: 1, minor: 0},
        }),
        "89d02f5c250b3e4a5bd70d00dd64ab0599285d6476f6b03fdea26ede80a1009e" => Ok(GameInfo {
            game_type: GameType::Deltarune,
            version: Version {major: 1, minor: 2},
        }),
        "e5299bfb361a681accf16f59efbac541a5fc0d343fcb3e75e72bcb8e54a4a9f9" => Ok(GameInfo {
            game_type: GameType::Deltarune,
            version: Version {major: 1, minor: 3},
        }),
        "05d34171189ae6913a32c07862c3a4294eabae45b62db9bb93595752fa55b756" => Ok(GameInfo {
            game_type: GameType::Deltarune,
            version: Version {major: 1, minor: 4},
        }),
        "22529512cca4a00b664ca028b55d3e7f9d7eeebdda6fa6e68237bbd2ed5f74f6" => Ok(GameInfo {
            game_type: GameType::Deltarune,
            version: Version {major: 1, minor: 5},
        }),
        "6a31fade424e742ec58a1564c38ce539e3a18c56655bb56c4b441c370ea16c86" => Ok(GameInfo {
            game_type: GameType::Deltarune,
            version: Version {major: 1, minor: 6},
        }),
        "614b46baece0079bfd4e6d0329deb61c3249badb1af1084cc513301faf279899" => Ok(GameInfo {
            game_type: GameType::Deltarune,
            version: Version {major: 1, minor: 7},
        }),
        "d1605525afdbac0a8f19ab0f8aa07e3ecc8b50ef7497f76106a0e19530d8c3ea" => Ok(GameInfo {
            game_type: GameType::Deltarune,
            version: Version {major: 1, minor: 8},
        }),
        "8b0ff2d0abcbd6c9c0f29e13015e6738b69af11ec4ec13596c822af93bb51081" => Ok(GameInfo {
            game_type: GameType::Deltarune,
            version: Version {major: 1, minor: 9},
        }),
        "9d945891d85fb83779f01b995ab84e44ac6952e63c74d2350c1e93087546f038" => Ok(GameInfo {
            game_type: GameType::Deltarune,
            version: Version {major: 1, minor: 10},
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

