use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local};
use iced::{alignment, Command, Element};
use iced::widget::{container, column, text, row, button, TextInput};
use crate::{Msg, MyApp, SceneType, COLOR_TEXT1, COLOR_TEXT2, COLOR_TEXT_RED, WINDOW_SIZE_VIEW_PROFILE};
use crate::default_file_paths::get_default_data_file_dir;
use crate::scenes::homepage::{load_profiles, Profile, SceneHomePage};
use crate::scenes::view_profile::SceneViewProfile;
use log::{info, warn};
use rfd::FileDialog;
use crate::scenes::create_profile::{detect_game_and_version, sanitize_profile_dir_name, resize_and_save_icon, SceneCreateProfile};
use crate::ui_templates::generate_button_bar;
use crate::utility::{GameInfo, GameVersion};

#[derive(Debug, Clone)]
pub enum MsgCreateProfile2 {
    BackToHomepage,
    StepBack,
    StepNext,
    EditDataPath(String),
    SubmitDataPath,
    PickDataPath,
    PickedDataPath(Option<PathBuf>),
    EditGameName(String),
    EditGameVersion(String),
}

impl SceneCreateProfile {
    pub fn update2(&mut self, app: &mut MyApp, message: Msg) -> Result<Command<Msg>, String> {
        let message: MsgCreateProfile2 = match message {
            Msg::CreateProfile2(msg) => msg,
            other => return Err(format!("Invalid message type {other:?} for CreateProfile2")),
        };

        match message {
            MsgCreateProfile2::BackToHomepage => {
                app.active_scene = SceneType::HomePage(SceneHomePage {});
            },

            MsgCreateProfile2::StepBack => {
                if self.is_file_picker_open {
                    return Err("Please close the file picker before changing scene.".to_string())
                }
                self.stage = 1;
            },

            MsgCreateProfile2::StepNext => {
                if self.is_file_picker_open {
                    return Err("Please close the file picker before changing scene.".to_string())
                }
                if !self.is_profile_name_valid 
                    || !self.is_game_version_valid
                    || self.game_info.game_name.trim().is_empty() 
                    || self.game_version_str.trim().is_empty() {
                    return Ok(Command::none())
                }
                return self.create_profile(app).map_err(|e| format!("Could not create profile: {e}"))
            }

            MsgCreateProfile2::EditDataPath(data_file_path) => {
                self.data_file_path = data_file_path;
            },

            MsgCreateProfile2::SubmitDataPath => {
                self.detect_game()?;
            },

            MsgCreateProfile2::PickDataPath => {
                if !self.is_file_picker_open {
                    self.is_file_picker_open = true;
                    return Ok(self.pick_data_path(app))
                }
            },

            MsgCreateProfile2::PickedDataPath(Some(data_path)) => {
                self.is_file_picker_open = false;
                let data_path: &str = data_path.to_str().ok_or_else(|| format!("Could not convert data path to string: {data_path:?}"))?;
                self.data_file_path = data_path.to_string();
                self.detect_game()?;
            },

            MsgCreateProfile2::PickedDataPath(None) => {
                self.is_file_picker_open = false;
                info!("User did not pick a data file and instead cancelled the operation.");
            },

            MsgCreateProfile2::EditGameName(name) => {
                // only allow editing when game couldn't be automatically detected
                if !self.game_auto_detected {
                    self.game_info.game_name = name;
                }
            },

            MsgCreateProfile2::EditGameVersion(version_str) => {
                // only allow editing when game couldn't be automatically detected
                if !self.game_auto_detected {
                    return Ok(Command::none())
                }
                self.game_version_str = version_str.trim().to_string();
                let Ok(version) = version_str.parse() else {
                    self.is_game_version_valid = false;
                    return Ok(Command::none())
                };
                self.game_info.game_version = version;
                self.is_game_version_valid = true;
            },
        }
        Ok(Command::none())
    }

    pub fn view2(&self, _app: &MyApp) -> Result<Element<Msg>, String> {
        let game_version_valid = text(
            if self.is_game_version_valid {""} else {"Invalid Version (example for valid version: 1.63)"}
        ).size(12).style(*COLOR_TEXT_RED);

        let auto_detected = text(if self.game_auto_detected {
            "Game Name and Version automatically detected!"
        } else {
            "Could not determine game version!\nIf your game is Undertale or Deltarune, please make sure it is not modified!"
        });

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
                            TextInput::new("Game", &self.game_info.game_name)
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
        
        let button_bar = generate_button_bar(vec![
            button("Cancel").on_press(Msg::CreateProfile2(MsgCreateProfile2::BackToHomepage)).into(),
            button("< Back").on_press(Msg::CreateProfile2(MsgCreateProfile2::StepBack)).into(),
        ], vec![
            button("Next >").on_press(Msg::CreateProfile2(MsgCreateProfile2::StepNext)).into(),
        ]);

        Ok(container(
            column![
                column![
                    main_content,
                ],
                button_bar
            ]
        ).into())
    }

    fn detect_game(&mut self) -> Result<(), String> {
        match detect_game_and_version(Path::new(&self.data_file_path))? {
            Some(game_info) => {
                self.game_version_str = game_info.game_version.to_string();
                self.game_info = game_info;
                self.game_auto_detected = true;
            }
            None => {
                self.game_version_str = "".to_string();
                self.game_info = GameInfo {
                    game_name: "".to_string(),
                    game_version: GameVersion::new(0, 0),
                };
                self.game_auto_detected = true;
            }
        }
        Ok(())
    }
}


impl SceneCreateProfile {
    fn create_profile(&mut self, app: &mut MyApp) -> Result<Command<Msg>, String> {
        let profile_dir_name: String = sanitize_profile_dir_name(&self.profile_name);
        let profile_dir: PathBuf = app.home_dir.join("profiles").join(profile_dir_name);

        if !profile_dir.exists() {
            fs::create_dir_all(&profile_dir).map_err(|e| format!("Could not create profile directory: {e}"))?;
        }

        let date_now: DateTime<Local> = Local::now();

        let profile = Profile {
            index: app.profiles.len(),
            name: self.profile_name.clone(),
            game_info: self.game_info.clone(),
            created_at: date_now,
            last_used: date_now,
            mods: vec![],
            icon: self.icon.clone(),
            path: profile_dir.clone(),
        };
        
        let config_file: PathBuf = profile_dir.join("profile.json");
        let config: String = serde_json::to_string_pretty(&profile)
            .map_err(|e| format!("Could not json serialize profile config: {e}"))?;

        // write config json
        fs::write(config_file, config)
            .map_err(|e| format!("Could not create profile config file: {e}"))?;
        
        // copy icon image
        let icon_path: PathBuf = profile_dir.join("icon.png");
        resize_and_save_icon(&self.icon, icon_path)?;

        // copy data win  |  {..} SLOW OPERATION
        let data_file: PathBuf = profile_dir.join("data.win");
        fs::copy(&self.data_file_path, data_file)
            .map_err(|e| format!("Could not copy data file: {e}"))?;

        // reload profiles for homepage
        app.profiles = load_profiles(&app.home_dir, false)?;
        
        app.active_scene = SceneType::ViewProfile(SceneViewProfile {
            profile,
            mods: vec![],
            browser: Default::default(),
            mod_details: Default::default(),
        });
        
        // resize window for new scene
        Ok(iced::window::resize(app.main_window_id, WINDOW_SIZE_VIEW_PROFILE))
    }

    fn pick_data_path(&mut self, app: &mut MyApp) -> Command<Msg> {
        let origin_path: PathBuf = get_default_data_file_dir().unwrap_or_else(|e| {
            warn!("Could not get default data file path: {e}");
            app.home_dir.clone()
        });

        let file_dialog: FileDialog = FileDialog::new()
            .set_title("Pick a GameMaker data file for your AcornGM profile")
            .set_directory(&origin_path)
            .add_filter("GameMaker Data File", &["win", "unx"]);

        Command::perform(
            async move { file_dialog.pick_file() },
            |i| Msg::CreateProfile2(MsgCreateProfile2::PickedDataPath(i))
        )
    }
}

