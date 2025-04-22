use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use iced::{alignment, Command, Element};
use iced::advanced::image::Data;
use iced::widget::{container, column, text, row, button, TextInput};
use image::DynamicImage;
use crate::{Msg, MyApp, SceneType, COLOR_TEXT1, COLOR_TEXT2, COLOR_TEXT_RED, WINDOW_SIZE_VIEW_PROFILE};
use crate::default_file_paths::{get_default_data_file_dir, show_msgbox};
use crate::scenes::homepage::{load_profiles, Profile, SceneHomePage};
use crate::scenes::view_profile::SceneViewProfile;
use crate::utility::{remove_spaces, GameType, Version};
use log::error;
use crate::scenes::create_profile::{detect_game_and_version, make_profile_dir_name_valid, resize_image_fast, SceneCreateProfile};

#[derive(Debug, Clone)]
pub enum MsgCreateProfile2 {
    BackToHomepage,
    StepBack,
    StepNext,
    EditDataPath(String),
    SubmitDataPath,
    PickDataPath,
    EditGameName(String),
    EditGameVersion(String),
}

impl SceneCreateProfile {
    pub fn update2(&mut self, app: &mut MyApp, message: Msg) -> Command<Msg> {
        let message: MsgCreateProfile2 = match message {
            Msg::CreateProfile2(msg) => msg,
            other => {
                error!("Invalid message type {other:?}");
                return Command::none()
            }
        };

        match message {
            MsgCreateProfile2::BackToHomepage => {
                app.active_scene = Arc::new(SceneType::HomePage(SceneHomePage {}));
            },

            MsgCreateProfile2::StepBack => {
                self.stage = 1;
            },

            MsgCreateProfile2::StepNext => {
                if !self.is_profile_name_valid || !self.is_game_version_valid {
                    return Command::none()
                }
                if let GameType::Unset = self.game_info.game_type {
                    return Command::none()
                }

                let profile_dir_name: String = make_profile_dir_name_valid(&self.profile_name);
                let profile_dir: PathBuf = app.home_dir.join(format!("./Profiles/{}", profile_dir_name));
                if let Err(error) = fs::create_dir_all(&profile_dir) {
                    show_msgbox("Error creating AcornGM profile", &format!("Could not create profile directory: {error}"))
                };

                let date_now: chrono::DateTime<chrono::Utc> = chrono::Utc::now();

                let profile = Profile {
                    index: app.profiles.len(),
                    name: self.profile_name.clone(),
                    game_info: self.game_info.clone(),
                    date_created: date_now.with_timezone(&chrono::Local),
                    last_used: date_now.with_timezone(&chrono::Local),
                    mods: vec![],
                    icon: self.icon.clone(),
                };

                // create config file
                let config_file: PathBuf = profile_dir.join("./profile.json");
                let date_string: String = date_now.to_string();
                let game_name: String = match &self.game_info.game_type {
                    GameType::Undertale => "Undertale".to_string(),
                    GameType::Deltarune => "Deltarune".to_string(),
                    GameType::Other(name) => name.clone(),
                    GameType::Unset => return Command::none(),
                };

                let config: serde_json::Value = serde_json::json!({
                    "displayName": self.profile_name,
                    "dateCreated": date_string,
                    "lastUsed": date_string,
                    "gameName": game_name,
                    "gameVersion": [self.game_info.version.major, self.game_info.version.minor],
                    "mods": [],
                });
                let config: String = serde_json::to_string_pretty(&config).unwrap();

                if let Err(error) = fs::write(config_file, config) {
                    show_msgbox("Error creating AcornGM profile", &format!("Could not create profile config file: {error}"))
                };

                // create icon file  | {..} SLOW OPERATION   TODO move to func and load directly with fir
                let icon_file: PathBuf = profile_dir.join("./icon.png");
                let image: DynamicImage = match self.icon.data() {
                    Data::Path(path) => {
                        image::open(path).unwrap_or_else(|error| {
                            show_msgbox("Error creating AcornGM profile",
                                        &format!("Could not create icon file because image::open could not parse Data::Path: {error}"));
                            DynamicImage::ImageRgba8(image::RgbaImage::new(1, 1))
                        })
                    }
                    Data::Bytes(bytes) => {
                        image::load_from_memory(&bytes).unwrap_or_else(|_| {
                            show_msgbox("Error creating AcornGM profile",
                                        "Could not create icon file because image::load_from_memory could not parse Data::Bytes.");
                            DynamicImage::ImageRgba8(image::RgbaImage::new(1, 1))
                        })
                    }
                    Data::Rgba { width, height, pixels } => {
                        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(*width, *height, pixels.to_vec()).unwrap_or_else(|| {
                            show_msgbox("Error creating AcornGM profile",
                                        "Could not create icon file because RgbaImage could not parse Data::Rgba.");
                            image::RgbaImage::new(1, 1)
                        }))
                    }
                };

                let resized_image: DynamicImage = resize_image_fast(image);     // cap resolution for performance | {..} SLOW OPERATION
                if let Err(error) = resized_image.save(icon_file) {
                    show_msgbox("Error creating AcornGM profile", &format!("Could not create profile icon file: {error}"))
                };

                // copy data win
                let data_file: PathBuf = profile_dir.join("./data.win");
                if let Err(error) = fs::copy(&self.data_file_path, data_file) {      // {..} SLOW OPERATION
                    show_msgbox("Error creating AcornGM profile", &format!("Could not copy data file: {error}"))
                };

                app.profiles = load_profiles(&app.home_dir);     // reload profiles for homepage
                app.active_scene = Arc::new(SceneType::ViewProfile(SceneViewProfile {
                    profile,
                    mods: vec![],
                    browser: Default::default(),
                    mod_details: Default::default(),
                }));
                return iced::window::resize(app.flags.main_window_id, WINDOW_SIZE_VIEW_PROFILE)
            }

            MsgCreateProfile2::EditDataPath(data_file_path) => {
                self.data_file_path = data_file_path;
            },

            MsgCreateProfile2::SubmitDataPath => {
                self.detect_game();
            },

            MsgCreateProfile2::PickDataPath => {
                let default_data_dir: PathBuf = match get_default_data_file_dir() {
                    Ok(path) => path,
                    Err(error) => {
                        println!("[WARN @ create_profile2::update]  Could not get default data file path: {error}"); return Command::none();
                    }
                };
                // this file picker blocks the main thread; causing it to appear as "Not responding"
                // perhaps use async {..}
                let data_path = native_dialog::FileDialog::new()
                    .set_location(&default_data_dir)
                    .add_filter("GameMaker Data File", &["win", "unx"])
                    .show_open_single_file();
                let data_path = match data_path {
                    Ok(p) => p,
                    Err(error) => { println!("[WARN @ create_profile2::update]  Could not get path from file picker: {}", error); return Command::none(); }
                };
                let data_path: PathBuf = match data_path {
                    Some(p) => p,
                    None => { println!("[WARN @ create_profile2::update]  Path from file picker is empty"); return Command::none(); }
                };
                let data_path: &str = match data_path.to_str() {
                    Some(p) => p,
                    None => { println!("[WARN @ create_profile2::update]  Could not convert data path to string"); return Command::none(); }
                };
                self.data_file_path = data_path.to_string();
                self.detect_game();
            },

            MsgCreateProfile2::EditGameName(name) => {
                match &self.game_info.game_type {
                    GameType::Other(_) => {
                        self.game_name = name.clone();
                        self.game_info.game_type = GameType::Other(name);
                    },
                    _ => {},
                }
            },

            MsgCreateProfile2::EditGameVersion(version_str) => {
                // ignore if no data file loaded or if version was automatically detected
                match self.game_info.game_type {
                    GameType::Other(_) => {},
                    _ => return Command::none(),
                }
                self.game_version_str = remove_spaces(&version_str);
                let version: Version = match version_str.parse() {
                    Ok(ver) => ver,
                    Err(_) => {
                        self.is_game_version_valid = false;
                        return Command::none();
                    }
                };
                self.game_info.version = version;
                self.is_game_version_valid = true;
            },
        }
        Command::none()
    }

    pub fn view2(&self, _app: &MyApp) -> Element<Msg> {
        let game_version_valid = text(
            if self.is_game_version_valid {""} else {"Invalid Version (example for valid version: 1.63)"}
        ).size(12).style(*COLOR_TEXT_RED);

        let auto_detected = text(
            match self.game_info.game_type {
                GameType::Unset => "",
                GameType::Other(_) => "Could not determine game version!\nIf your game is Undertale or Deltarune, please make sure it is not modified!",
                _ => "Game Name and Version automatically detected!",
            }
        );

        let main_content = container(
            iced::widget::column![
                column![
                    text("Create New Profile").size(22).style(*COLOR_TEXT1),
                    text("").size(10),
                    text("GameMaker Data File").size(14).style(*COLOR_TEXT2),
                    text("").size(4),
                    row![
                        TextInput::new(
                            "/path/to/data.win",
                            &self.data_file_path
                        )
                            .on_input(|string| Msg::CreateProfile2(MsgCreateProfile2::EditDataPath(string)))
                            .on_submit(Msg::CreateProfile2(MsgCreateProfile2::SubmitDataPath)),
                        button("Pick File").on_press(Msg::CreateProfile2(MsgCreateProfile2::PickDataPath)),
                    ].spacing(10),
                    text("").size(16),
                    row![
                        column![
                            text("Game Name").size(14).style(*COLOR_TEXT2),
                            text("").size(4),
                            TextInput::new("Game", &self.game_name)
                                .on_input(|string| Msg::CreateProfile2(MsgCreateProfile2::EditGameName(string)))
                        ],
                        column![
                            text("Game Version").size(14).style(*COLOR_TEXT2),
                            text("").size(4),
                            TextInput::new("Version", &self.game_version_str)
                                .on_input(|string| Msg::CreateProfile2(MsgCreateProfile2::EditGameVersion(string))),
                            text("").size(4),
                            game_version_valid,
                        ],
                    ].spacing(69),
                    text("").size(15),
                    auto_detected,
                ]
                .padding(20)
            ]
        ).align_x(alignment::Horizontal::Left);

        let button_bar = container(
            row![
                container(
                    row![
                        text("    ").size(10),
                        button("Cancel").on_press(Msg::CreateProfile2(MsgCreateProfile2::BackToHomepage)),
                        button("< Back").on_press(Msg::CreateProfile2(MsgCreateProfile2::StepBack)),
                    ]
                    .spacing(10)
                ),
                text("                                                        ").size(20),
                container(
                     row![
                        button("Next >").on_press(Msg::CreateProfile2(MsgCreateProfile2::StepNext)),
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

    fn detect_game(&mut self) {
        let data_file_path: &Path = Path::new(&self.data_file_path);

        match detect_game_and_version(data_file_path) {
            Ok(game_info) => {
                self.game_name = match &game_info.game_type {
                    GameType::Other(name) => name.clone(),
                    GameType::Undertale => "Undertale".to_string(),
                    GameType::Deltarune => "Deltarune".to_string(),
                    GameType::Unset => "".to_string(),
                };
                self.game_version_str = game_info.version.to_string();
                self.game_info = game_info;
            },
            Err(_) => {},
        };
    }
}


