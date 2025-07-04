mod homepage1;

use std::fs;
use std::fs::ReadDir;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local, Utc};
use iced::{Color, Element, Length};
use iced::widget::{button, column, container, row, text, Image, Space};
use iced::widget::image::Handle;
use log::warn;
use crate::Msg;
use crate::utility::{GameInfo, TransparentButton, GameVersion};
use serde;
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::scenes::view_profile::{AcornModLocal};
use crate::ui_templates::item_style;

#[derive(Debug, Clone)]
pub enum MsgHomePage {
    CreateProfile,
    LoadProfile(usize),
    Login,
}

#[derive(Debug, Clone)]
pub struct SceneHomePage {
    pub update_status_text: &'static str,
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub index: usize,                   // index in .profiles to identify profile on press
    pub name: String,
    pub game_info: GameInfo,
    pub created_at: DateTime<Local>,
    pub last_used: DateTime<Local>,
    pub mods: Vec<AcornModLocal>,
    pub icon: Handle,
    pub path: PathBuf,
}

impl Serialize for Profile {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut profile = serializer.serialize_struct("Profile", 6)?;
        profile.serialize_field("name", &self.name)?;
        profile.serialize_field("game_name", &self.game_info.game_name)?;
        profile.serialize_field("game_version", &[self.game_info.game_version.major, self.game_info.game_version.minor])?;
        profile.serialize_field("created_at", &self.created_at.to_utc())?;
        profile.serialize_field("last_used", &self.last_used.to_utc())?;
        profile.serialize_field("mods", &self.mods)?;
        profile.end()
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
                    Space::with_height(8.0),
                    row![
                        Space::with_width(8.0),
                        icon.width(50),
                        Space::with_width(12.0),
                        column![
                            row![
                                text(&self.name).size(16).style(color_text1),
                                Space::with_width(8.0),
                                column![
                                    Space::with_height(4.0),
                                    text(self.last_used.format("%Y-%m-%d %H:%M")).size(12).style(color_text2),
                                ],
                            ],
                            Space::with_height(5.0),
                            text(format!("{}/{} Mods Loaded", active_mod_count, self.mods.len())).size(13).style(color_text1),
                        ]
                    ]
                ]
            )
                .style(iced::theme::Button::Custom(Box::new(TransparentButton)))
                .on_press(Msg::HomePage(MsgHomePage::LoadProfile(self.index)))
                .width(Length::Fill)
                .height(Length::Fill)
        )
            .style(item_style)
            .height(65)
            .into()
    }
}

pub fn load_profiles(home_dir: &Path, is_first_launch: bool) -> Result<Vec<Profile>, String> {
    let profiles_dir: PathBuf = home_dir.join("profiles");

    if is_first_launch && !profiles_dir.is_dir() {
        fs::create_dir_all(profiles_dir)
            .map_err(|e| format!("Could not create profiles directory: {e}"))?;
        return Ok(vec![])   // return empty list for profiles on first launch instead of error message
    }

    let paths: ReadDir = fs::read_dir(&profiles_dir)
        .map_err(|e| format!("Could not get files in Profiles directory: {e}"))?;

    let mut profiles: Vec<Profile> = vec![];

    for path in paths {
        let path: PathBuf = path
            .map_err(|e| format!("Could not unwrap files in Profiles directory: {e}"))?
            .path();

        if !path.is_dir() {
            continue
        }

        match load_profile(path, profiles.len()) {
            Ok(profile) => profiles.push(profile),
            Err(e) => warn!("{e}"),
        }
    }

    profiles.sort_by(|a, b| b.last_used.cmp(&a.last_used));
    Ok(profiles)
}

fn load_profile(profile_dir: PathBuf, index: usize) -> Result<Profile, String> {
    #[derive(Deserialize)]
    struct ProfileHelper {
        pub name: String,
        pub game_name: String,
        pub game_version: [u32; 2],
        pub created_at: DateTime<Utc>,
        pub last_used: DateTime<Utc>,
        pub mods: Vec<AcornModLocal>,
    }

    let config_file: PathBuf = profile_dir.join("profile.json");
    let icon_file: PathBuf = profile_dir.join("icon.png");

    let config: String = fs::read_to_string(&config_file)
        .map_err(|e| format!("Could not read config file of Profile \"{}\": {e}", config_file.display()))?;

    let profile_helper: ProfileHelper = serde_json::from_str(&config)
        .map_err(|e| format!("Could not parse config file of Profile \"{}\": {e}", config_file.display()))?;

    Ok(Profile {
        index,
        name: profile_helper.name.clone(),
        game_info: GameInfo {
            game_name: profile_helper.game_name,
            game_version: GameVersion::new(profile_helper.game_version[0], profile_helper.game_version[1])
        },
        created_at: DateTime::from(profile_helper.created_at),
        last_used: DateTime::from(profile_helper.last_used),
        mods: profile_helper.mods,
        icon: Handle::from_path(icon_file),
        path: profile_dir,
    })
}


pub fn update_profile_config(profile: &Profile) -> Result<(), String> {
    let string = serde_json::to_string_pretty(&profile)
        .map_err(|e| format!("Could not json serialize profile config: {e}"))?;
    let path: PathBuf = profile.path.join("profile.json");
    fs::write(path, string).map_err(|e| format!("Could not write profile config file: {e}"))?;
    Ok(())
}

