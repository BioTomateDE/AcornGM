use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;
use iced::{alignment, Color, Command, Element};
use iced::widget::{button, column, container, row, scrollable, text, Container, Image};
use iced::widget::container::Appearance;
use iced::widget::image::Handle;
use crate::{Msg, MyApp, SceneCreateProfile, SceneType, WINDOW_SIZE_VIEW_PROFILE};
use crate::default_file_paths::{show_msgbox};
use crate::utility::{get_default_icon_image, GameInfo, GameType, TransparentButton, Version};
use serde;
use crate::scenes::browser::ModBrowser;
use crate::scenes::login::SceneLogin;
use crate::scenes::view_profile::{AcornMod, SceneViewProfile};

#[derive(Debug, Clone)]
pub enum MsgHomePage {
    ProfilesLoaded(Vec<Profile>),
    CreateProfile,
    LoadProfile(usize),
    Login,
}


#[derive(Debug, Clone)]
pub struct SceneHomePage;

impl MyApp {
    pub fn update_homepage(&mut self, message: Msg) -> Command<Msg> {
        match message {
            Msg::HomePage(MsgHomePage::CreateProfile) => {
                self.active_scene = SceneType::CreateProfile1(SceneCreateProfile {
                    profile_name: "My Profile".to_string(),
                    is_profile_name_valid: true,
                    icon: get_default_icon_image(&self.current_working_dir),
                    data_file_path: "".to_string(),
                    game_info: GameInfo::default(),
                    game_name: "".to_string(),
                    game_version_str: "".to_string(),
                    is_game_version_valid: true,        // to hide error when no data file is loaded
                });
            },

            Msg::HomePage(MsgHomePage::LoadProfile(index)) => {
                // load wingdings font
                if let Some(profile) = self.profiles.get(index) {
                    self.active_scene = SceneType::ViewProfile(SceneViewProfile {
                        mods: vec![],
                        profile: profile.clone(),
                        browser: ModBrowser {
                            search_query: "".to_string(),
                            use_regex: false,
                            results: vec![],
                            show_only_compatible: true,
                        },
                    })
                }
                // let command: Command<Result<(), !>> = match get_local_font("wingdings") {
                //     Ok(command) => command,
                //     Err(error) => {
                //         show_msgbox("Error while loading font", &format!("Could not load wingdings font: {error}"));
                //         return Command::none()
                //     }
                // };
                // return command
                return iced::window::resize(self.flags.main_window_id, WINDOW_SIZE_VIEW_PROFILE)
            },

            Msg::HomePage(MsgHomePage::Login) => {
                self.active_scene = SceneType::Login(SceneLogin {
                    temp_login_token: None,
                    status_string: "Idle",
                });
            },

            _ => {},
        }
        Command::none()
    }

    pub fn view_homepage(&self) -> Element<Msg> {
        let mut profiles: Vec<Element<Msg>> = Vec::new();
        for (_i, profile) in self.profiles.iter().enumerate() {
            profiles.push(profile.view(self.color_text1, self.color_text2));
            // if i != self.profiles.len() - 1 {    // if not last elem, push divider
            //     profiles.push(create_divider())
            // }
            profiles.push(create_divider())
        }
        let profiles: Container<Msg> = container(column(profiles).spacing(5)).style(list_style);

        let main_content = container(
            iced::widget::column![
                column![
                    text("").size(10),
                    text("AcornGM").size(28).style(self.color_text1),
                    text("").size(6),
                    text("Recent Profiles").size(14).style(self.color_text2).horizontal_alignment(alignment::Horizontal::Center),
                    text("").size(6),
                    scrollable(profiles).height(500),
                ]
                .padding(20)
            ]
        ).align_x(alignment::Horizontal::Left);

        let button_bar = container(
            row![
                container(
                    row![
                        text("    ").size(10),
                        button("Login").on_press(Msg::HomePage(MsgHomePage::Login)),
                    ]
                    .spacing(10)
                ),
                text("                                                               ").size(19),
                container(
                     row![
                        button("Create Profile").on_press(Msg::HomePage(MsgHomePage::CreateProfile)),
                        text("    ").size(10),
                    ]
                    .spacing(10)
                )
            ]
        )
            .width(900);

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

#[derive(Debug, Clone)]
pub struct Profile {
    pub index: usize,                   // index in .profiles to identify profile on press
    pub name: String,
    pub game_info: GameInfo,
    pub date_created: chrono::DateTime<chrono::Local>,
    pub last_used: chrono::DateTime<chrono::Local>,
    pub mods: Vec<ModReference>,
    pub icon: Handle,
    pub gm_data: Option<Vec<u8>>,       // not set in homepage; only on load
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
            gm_data: None,
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



pub fn load_profiles(home_dir: &PathBuf) -> Vec<Profile> {
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
        let config_file: PathBuf = path.join("./profile.json");
        let icon_file: PathBuf = path.join("./icon.png");

        let config: String = match fs::read_to_string(&config_file) {
            Ok(ok) => ok,
            Err(error) => {
                show_msgbox("Error while getting profiles", &format!(
                    "Could not read config file of Profile \"{}\": {error}", config_file.to_str().unwrap_or_else(||""),
                ));
                continue;
            }
        };

        let profile_json: ProfileJson = match serde_json::from_str(&config) {
            Ok(ok) => ok,
            Err(error) => {
                show_msgbox("Error while getting profiles", &format!(
                    "Could not read config file of Profile \"{:?}\": {error}", config_file.to_str().unwrap_or_else(||""),
                ));
                continue;
            }
        };

        let game_type: GameType = match profile_json.game_name.as_str() {
            "Undertale" => GameType::Undertale,
            "Deltarune" => GameType::Deltarune,
            other => GameType::Other(other.to_string()),
        };
        let game_version = Version { major: profile_json.game_version[0], minor: profile_json.game_version[1] };
        let game_info: GameInfo = GameInfo { game_type, version: game_version };
        let date_created: chrono::DateTime<chrono::Local> = match profile_json.date_created.parse() {
            Ok(ok) => ok,
            Err(error) => {
                show_msgbox("Error while getting profiles", &format!(
                    "Could not parse creation datetime \"{}\" of Profile \"{:?}\": {}", profile_json.date_created, path.to_str(), error,
                ));
                continue;
            }
        };
        let last_used: chrono::DateTime<chrono::Local> = match profile_json.date_created.parse() {
            Ok(ok) => ok,
            Err(error) => {
                show_msgbox("Error while getting profiles", &format!(
                    "Could not parse last used datetime \"{}\" of Profile \"{:?}\": {}", profile_json.last_used, path.to_str(), error,
                ));
                continue;
            }
        };

        // maybe check if icon exists
        let icon: Handle = Handle::from_path(icon_file);

        profiles.push(Profile {
            index: profiles.len(),
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

