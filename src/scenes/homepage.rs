#![allow(unused_imports)]

mod homepage1;

use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;
use chrono::{DateTime, Local, Utc};
use iced::{alignment, Color, Element, Length};
use iced::widget::{button, column, container, row, text, Container, Image, Space};
use iced::widget::container::Appearance;
use iced::widget::image::Handle;
use log::warn;
use crate::Msg;
use crate::utility::{GameInfo, GameType, TransparentButton, Version};
use serde;
use crate::scenes::view_profile::{AcornMod, AcornModLocal};
use crate::ui_templates::item_style;

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
    pub date_created: DateTime<Local>,
    pub last_used: DateTime<Local>,
    pub mods: Vec<AcornModLocal>,
    pub icon: Handle,
    pub path: PathBuf,
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


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ProfileJson {
    display_name: String,
    date_created: String,
    last_used: String,
    game_name: String,
    game_version: [u32; 2],
    mods: Vec<String>,      // list of mods' uuids, sorted by descending priority
}

pub fn load_profiles(home_dir: &PathBuf, is_first_launch: bool) -> Result<Vec<Profile>, String> {
    let profiles_dir: PathBuf = home_dir.join("Profiles");

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

        if !path.is_dir() { continue }
        let config_file: PathBuf = path.join("profile.json");
        let icon_file: PathBuf = path.join("icon.png");

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
        let date_created: DateTime<Local> = match profile_json.date_created.parse() {
            Ok(date) => date,
            Err(e) => {
                warn!("Could not parse creation datetime \"{}\" of Profile \"{}\": {e}", profile_json.date_created, path.display());
                continue
            }
        };
        let last_used: DateTime<Local> = match profile_json.last_used.parse() {
            Ok(ok) => ok,
            Err(e) => {
                warn!("Could not parse last used datetime \"{}\" of Profile \"{}\": {e}", profile_json.last_used, path.display());
                continue
            }
        };

        // maybe check if icon exists?
        let icon: Handle = Handle::from_path(icon_file);

        let mods: Vec<AcornModLocal> = load_profile_mods(&path, profile_json.mods).unwrap_or_else(|e| {
            warn!("Could not load mods of profile {path:?}: {e}");
            vec![]
        });

        profiles.push(Profile {
            index: profiles.len(),
            name: profile_json.display_name,
            game_info,
            date_created,
            last_used,
            mods,
            icon,
            path,
        })
    }

    profiles.sort_by(|a, b| b.last_used.cmp(&a.last_used));
    Ok(profiles)
}

fn load_profile_mods(profile_dir: &PathBuf, mod_ids: Vec<String>) -> Result<Vec<AcornModLocal>, String> {
    let mods_dir: PathBuf = profile_dir.join("Mods");
    if !mods_dir.is_dir() {
        return Err(format!("Profile mods directory doesn't exist: {mods_dir:?}"))
    }

    let acorn_mods: Vec<AcornModLocal> = Vec::new();

    for mod_id in mod_ids {
        let _mod_path: PathBuf = mods_dir.join(format!("{mod_id}.acornmod"));
        // TODO read mod file as zip and extract information
    }

    Ok(acorn_mods)
}


pub fn update_profile_config(profile: &Profile) -> Result<(), String> {
    let mod_ids: Vec<String> = profile.mods.iter().map(|i| i.filename.clone()).collect();

    let profile_json = serde_json::json!({
        "displayName": profile.name,
        "dateCreated": profile.date_created.to_utc().to_string(),
        "lastUsed": Utc::now().to_string(),
        "gameName": profile.game_info.game_type.to_string(),
        "gameVersion": [profile.game_info.version.major, profile.game_info.version.minor],
        "mods": mod_ids,
    });

    let string = serde_json::to_string_pretty(&profile_json)
        .map_err(|e| format!("Could not convert profile config json to string: {e}"))?;

    let path: PathBuf = profile.path.join("profile.json");
    fs::write(path, string)
        .map_err(|e| format!("Could not write profile config file: {e}"))?;

    Ok(())
}

