use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;
use iced::{alignment, Element};
use iced::widget::{button, column, container, row, scrollable, text, Column};
use crate::{Msg, SceneCreateProfile, SceneMain, SceneType};
use crate::default_file_paths::{get_home_directory, show_msgbox};
use crate::utility::{get_default_icon_image, GameInfo, GameType};
use serde;

#[derive(Debug, Clone)]
pub enum MsgHomePage {
    CreateProfile,
}


#[derive(Debug, Clone)]
pub struct SceneHomePage {
    pub profiles: Vec<Profile>,
    pub profiles_loading_state: ProfilesLoadingState,
}
impl Default for SceneHomePage {
    fn default() -> Self {
        SceneHomePage {
            profiles: vec![],
            profiles_loading_state: ProfilesLoadingState::NotLoaded,
        }
    }
}


impl SceneMain {
    pub fn update_homepage(&mut self, message: Msg) {
        let _scene: &mut SceneHomePage = match &mut self.active_scene {
            SceneType::HomePage(scene) => scene,
            _ => {
                println!("[ERROR @ homepage::update]  Could not extract scene: {:?}", self.active_scene);
                return;
            }
        };

        // Task::run(load_profiles, |profiles| {scene.profiles = profiles}).await;
        match message {
            Msg::HomePage(MsgHomePage::CreateProfile) => {
                self.active_scene = SceneType::CreateProfile1(SceneCreateProfile {
                    profile_name: "My Profile".to_string(),
                    is_profile_name_valid: true,
                    icon: get_default_icon_image(),
                    data_file_path: "".to_string(),
                    game_info: GameInfo::default(),
                    game_name: "".to_string(),
                });
            },
            _ => {},
        }
    }

    pub fn view_homepage<'a>(&self, scene_homepage: &'a SceneHomePage) -> Element<'a, Msg> {
        let profiles: Column<Msg> = column(
            scene_homepage.profiles.iter().map(
                |i| i.view()
            )
        );

        let main_content = container(
            iced::widget::column![
                column![
                    text("").size(10),
                    text("AcornGM").size(28).style(self.color_text1),
                    text("").size(6),
                    text("Recent Profiles").size(12).style(self.color_text2).horizontal_alignment(alignment::Horizontal::Center),
                    scrollable(profiles).height(100),
                    // text("").size(18),
                ]
                .padding(20)
            ]
        ).align_x(alignment::Horizontal::Left);

        let button_bar = container(
            row![
                button("Create Profile").on_press(Msg::HomePage(MsgHomePage::CreateProfile)),
                button("Sample Text"),
                button("Lorem ipsum"),
                text("    ").size(10)
            ]
                .spacing(10)
        )
            .width(900)
            .align_x(alignment::Horizontal::Right);

        container(
            column![
                column![
                    main_content,
                ]
                .height(460),
                button_bar
            ]
        )
            .into()
    }
}


#[derive(Default, Debug, Clone)]
pub enum ProfilesLoadingState {
    #[default]
    NotLoaded,
    CurrentlyLoading,
    Loaded,
}

#[derive(Default, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub game_info: GameInfo,
    pub date_created: chrono::DateTime<chrono::Local>,
    pub last_used: chrono::DateTime<chrono::Local>,
    pub mods: Vec<ModReference>,
    pub icon: image::DynamicImage,
    pub gm_data: Option<Vec<u8>>,       // not set in homepage; only on load
}
impl Profile {
    fn view(&self) -> Element<Msg> {
        container(
            row![text(&self.name), text(self.last_used.to_string())]
        ).into()
    }
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ProfileJson {
    display_name: String,
    date_created: String,
    last_used: String,
    game_name: String,
    game_version: String,
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



async fn load_profiles() -> Vec<Profile> {
    let home_dir: PathBuf = get_home_directory();
    let profiles_dir: PathBuf = home_dir.join("./Profiles");

    let paths: ReadDir = match fs::read_dir(profiles_dir) {
        Ok(ok) => ok,
        Err(error) => {
            show_msgbox("Error while getting profiles", &format!("Could not get files in Profiles directory: {error}"));
            return vec![];
        }
    };

    let mut profiles: Vec<Profile> = vec![];

    for path in paths {
        let path = match path {
            Ok(ok) => ok,
            Err(error) => {
                show_msgbox("Error while getting profiles", &format!("Could not unwrap files in Profiles directory: {error}"));
                return vec![];
            }
        };

        let path: PathBuf = path.path();
        if !path.is_dir() { continue }
        let config_file: PathBuf = path.join("./config.json");
        let icon_file: PathBuf = path.join("./icon.png");

        let config: String = match fs::read_to_string(config_file) {
            Ok(ok) => ok,
            Err(error) => {
                show_msgbox("Error while getting profiles", &format!("Could not read config file of Profile \"{:?}\": {error}", path.to_str()));
                return vec![];
            }
        };

        let profile_json: ProfileJson = match serde_json::from_str(&config) {
            Ok(ok) => ok,
            Err(error) => {
                show_msgbox("Error while getting profiles", &format!("Could not read config file of Profile \"{:?}\": {error}", path.to_str()));
                return vec![];
            }
        };

        let game_type: GameType = match profile_json.game_name.as_str() {
            "Undertale" => GameType::Undertale,
            "Deltarune" => GameType::Deltarune,
            other => GameType::Other(other.to_string()),
        };
        let game_info: GameInfo = GameInfo { game_type, version: profile_json.game_version };
        let date_created: chrono::DateTime<chrono::Local> = match profile_json.date_created.parse() {
            Ok(ok) => ok,
            Err(error) => {
                show_msgbox("Error while getting profiles", &format!(
                    "Could not parse creation datetime \"{}\" of Profile \"{:?}\": {}", profile_json.date_created, path.to_str(), error,
                ));
                return vec![];
            }
        };
        let last_used: chrono::DateTime<chrono::Local> = match profile_json.date_created.parse() {
            Ok(ok) => ok,
            Err(error) => {
                show_msgbox("Error while getting profiles", &format!(
                    "Could not parse last used datetime \"{}\" of Profile \"{:?}\": {}", profile_json.last_used, path.to_str(), error,
                ));
                return vec![];
            }
        };

        let icon: image::DynamicImage = match image::open(icon_file) {
            Ok(ok) => ok,
            Err(error) => {
                show_msgbox("Error while getting profiles", &format!("Could not read icon file of Profile \"{:?}\": {error}", path.to_str()));
                return vec![];
            }
        };

        profiles.push(Profile {
            name: profile_json.display_name,
            game_info,
            date_created,
            last_used,
            mods: profile_json.mods,
            icon,
            gm_data: None,
        })
    }
    profiles
}

