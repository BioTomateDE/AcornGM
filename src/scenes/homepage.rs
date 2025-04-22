#![allow(unused_imports)]

mod homepage1;

use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;
use iced::{Color, Element};
use iced::widget::{button, column, container, row, text, Container, Image};
use iced::widget::container::Appearance;
use iced::widget::image::Handle;
use log::warn;
use crate::Msg;
use crate::utility::{GameInfo, GameType, TransparentButton, Version};
use serde;

#[derive(Debug, Clone)]
pub enum MsgHomePage {
    CreateProfile,
    LoadProfile(usize),
    Login,
}

#[derive(Debug, Clone)]
pub struct SceneHomePage;

#[derive(Debug, Clone)]
pub struct Profile {
    pub index: usize,                   // index in .profiles to identify profile on press
    pub name: String,
    pub game_info: GameInfo,
    pub date_created: chrono::DateTime<chrono::Local>,
    pub last_used: chrono::DateTime<chrono::Local>,
    pub mods: Vec<ModReference>,
    pub icon: Handle,
}
impl Default for Profile {
    fn default() -> Self {
        Self {
            index: 0,
            name: "Unknown Profile".to_string(),
            game_info: Default::default(),
            date_created: Default::default(),
            last_used: Default::default(),
            mods: vec![],
            icon: Handle::from_pixels(1, 1, [0, 0, 0, 0]),
        }
    }
}
impl Profile {
    fn view(&self, color_text1: Color, color_text2: Color) -> Element<Msg> {
        let icon: Image<Handle> = Image::new(self.icon.clone());
        let mut active_mod_count: usize = 0;
        for mod_ref in &self.mods {
            if mod_ref.active {
                active_mod_count += 1;
            }
        }

        container(
            button(
                column![
                    text("").size(10),
                    row![
                        text("   ").size(20),
                        icon.width(50),
                        text("    ").size(10),
                        column![
                            row![
                                text(&self.name).size(16).style(color_text1),
                                text("      ").size(10),
                                column![
                                    text("").size(4),
                                    text(self.last_used.format("%Y-%m-%d %H:%M")).size(12).style(color_text2),
                                ],
                            ],
                            text("").size(6),
                            text(format!("{}/{} Mods Loaded", active_mod_count, self.mods.len())).size(13).style(color_text1),
                        ]
                    ]
                ]
            )
                .style(iced::theme::Button::Custom(Box::new(TransparentButton)))
                .on_press(Msg::HomePage(MsgHomePage::LoadProfile(self.index)))
        )
            .width(700)
            .style(item_style)
            .height(80)
            .into()
    }
}


fn divider_style(_theme: &iced::Theme) -> Appearance {
    Appearance {
        background: Some(iced::Background::Color(Color::from_rgb8(34, 33, 31))),
        border: iced::Border {
            color: Color::TRANSPARENT,                  // No border for divider
            width: 0.0,                                 // No actual border width
            radius: iced::border::Radius::from(0),  // No border radius
        },
        shadow: Default::default(),
        text_color: None,
    }
}
pub fn create_divider() -> Element<'static, Msg> {
    Container::new(text(""))
        .height(0.75)                   // Height of the divider
        .width(iced::Length::Fill)      // Full width
        .center_x()                     // Center horizontally
        .style(divider_style)
        .into()
}

pub fn item_style(_theme: &iced::Theme) -> Appearance {
    Appearance {
        text_color: None,
        background: None,
        border: iced::Border::default(),
        shadow: Default::default(),
    }
}
pub fn list_style(_theme: &iced::Theme) -> Appearance {
    Appearance {
        text_color: None,
        background: Some(iced::Background::Color(Color::from_rgb8(47, 47, 46))),
        border: iced::Border {
            color: Color::from_rgb8(34, 33, 31),
            width: 2.0,
            radius: iced::border::Radius::from([0.0, 0.0, 0.0, 0.0]),
        },
        shadow: Default::default(),
    }
}



#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ProfileJson {
    display_name: String,
    date_created: String,
    last_used: String,
    game_name: String,
    game_version: [u32; 2],
    mods: Vec<ModReference>,
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModReference {
    pub name: String,
    pub by: String,
    pub version: String,
    pub active: bool,
}



pub fn load_profiles(home_dir: &PathBuf) -> Result<Vec<Profile>, String> {
    // check if acorn home dir exists to prevent error message on first launch
    if !home_dir.is_dir() {
        warn!("Did not load profiles because the AcornGM home directory does not exist.\
            This should only happen on the first launch when no profiles are created yet.");
        return Ok(vec![])
    }
    
    let profiles_dir: PathBuf = home_dir.join("./Profiles");
    let paths: ReadDir = fs::read_dir(profiles_dir)
        .map_err(|e| format!("Could not get files in Profiles directory: {e}"))?;

    let mut profiles: Vec<Profile> = vec![];

    for path in paths {
        let path: PathBuf = path
            .map_err(|e| format!("Could not unwrap files in Profiles directory: {e}"))?
            .path();

        if !path.is_dir() { continue }
        let config_file: PathBuf = path.join("./profile.json");
        let icon_file: PathBuf = path.join("./icon.png");

        let config: String = match fs::read_to_string(&config_file) {
            Ok(cfg) => cfg,
            Err(e) => {
                warn!("Could not read config file of Profile \"{}\": {e}", config_file.display());
                continue
            }
        };

        let profile_json: ProfileJson = match serde_json::from_str(&config) {
            Ok(json) => json,
            Err(e) => {
                warn!("Could not parse config file of Profile \"{}\": {e}", config_file.display());
                continue
            }
        };

        let game_type: GameType = GameType::from_name(&profile_json.game_name);
        let game_version: Version = Version::from_vec(profile_json.game_version);
        let game_info: GameInfo = GameInfo { game_type, version: game_version };
        let date_created: chrono::DateTime<chrono::Local> = match profile_json.date_created.parse() {
            Ok(date) => date,
            Err(e) => {
                warn!("Could not parse creation datetime \"{}\" of Profile \"{}\": {e}", profile_json.date_created, path.display());
                continue
            }
        };
        let last_used: chrono::DateTime<chrono::Local> = match profile_json.date_created.parse() {
            Ok(ok) => ok,
            Err(e) => {
                warn!("Could not parse last used datetime \"{}\" of Profile \"{}\": {e}", profile_json.last_used, path.display());
                continue
            }
        };

        // maybe check if icon exists?
        let icon: Handle = Handle::from_path(icon_file);

        profiles.push(Profile {
            index: profiles.len(),
            name: profile_json.display_name,
            game_info,
            date_created,
            last_used,
            mods: profile_json.mods,
            icon,
        })
    }
    Ok(profiles)
}

