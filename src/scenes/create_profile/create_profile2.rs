use std::fs;
use std::path::{Path, PathBuf};
use iced::{alignment, Command, Element};
use iced::widget::{container, column, text, row, button, TextInput};
use crate::{Msg, MyApp, SceneType, COLOR_TEXT1, COLOR_TEXT2, COLOR_TEXT_RED, WINDOW_SIZE_VIEW_PROFILE};
use crate::default_file_paths::get_default_data_file_dir;
use crate::scenes::homepage::{load_profiles, Profile, SceneHomePage};
use crate::scenes::view_profile::SceneViewProfile;
use crate::utility::{remove_spaces, show_error_dialogue, GameType};
use log::{error, info, warn};
use rfd::FileDialog;
use crate::scenes::create_profile::{detect_game_and_version, make_profile_dir_name_valid, resize_and_save_icon, SceneCreateProfile};
use crate::ui_templates::generate_button_bar;

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
                app.active_scene = SceneType::HomePage(SceneHomePage {});
            },

            MsgCreateProfile2::StepBack => {
                if self.is_file_picker_open {
                    show_error_dialogue("AcornGM User Error", "Please close the file picker before changing scene.");
                    return Command::none()
                }
                self.stage = 1;
            },

            MsgCreateProfile2::StepNext => {
                if self.is_file_picker_open {
                    show_error_dialogue("AcornGM User Error", "Please close the file picker before changing scene.");
                    return Command::none()
                }
                if !self.is_profile_name_valid || !self.is_game_version_valid {
                    return Command::none()
                }
                if let GameType::Unset = self.game_info.game_type {
                    return Command::none()
                }
                return self.create_profile(app).unwrap_or_else(|e| {
                    show_error_dialogue("Could not create AcornGM profile", &e);
                    Command::none()
                })
            }

            MsgCreateProfile2::EditDataPath(data_file_path) => {
                self.data_file_path = data_file_path;
            },

            MsgCreateProfile2::SubmitDataPath => {
                self.detect_game();
            },

            MsgCreateProfile2::PickDataPath => {
                if !self.is_file_picker_open {
                    self.is_file_picker_open = true;
                    return self.pick_data_path(app)
                }
            },

            MsgCreateProfile2::PickedDataPath(Some(data_path)) => {
                self.is_file_picker_open = false;
                let data_path: &str = match data_path.to_str() {
                    Some(string) => string,
                    None => {
                        error!("Could not convert data path to string: {data_path:?}");
                        return Command::none()
                    }
                };
                self.data_file_path = data_path.to_string();
                self.detect_game();
            },

            MsgCreateProfile2::PickedDataPath(None) => {
                self.is_file_picker_open = false;
                info!("User did not pick a data file and instead cancelled the operation.");
            },

            MsgCreateProfile2::EditGameName(name) => {
                // only allow for `Other` game type; when game wasn't automatically detected
                if let GameType::Other(_) = &self.game_info.game_type {
                    self.game_name = name.clone();
                    self.game_info.game_type = GameType::Other(name);
                }
            },

            MsgCreateProfile2::EditGameVersion(version_str) => {
                // ignore if no data file loaded or if version was automatically detected
                if !matches!(self.game_info.game_type, GameType::Other(_)) {
                    return Command::none()
                }
                self.game_version_str = remove_spaces(&version_str);
                let Ok(version) = version_str.parse() else {
                    self.is_game_version_valid = false;
                    return Command::none();
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
        
        let button_bar = generate_button_bar(vec![
            button("Cancel").on_press(Msg::CreateProfile2(MsgCreateProfile2::BackToHomepage)).into(),
            button("< Back").on_press(Msg::CreateProfile2(MsgCreateProfile2::StepBack)).into(),
        ], vec![
            button("Next >").on_press(Msg::CreateProfile2(MsgCreateProfile2::StepNext)).into(),
        ]);

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


impl SceneCreateProfile {
    fn create_profile(&mut self, app: &mut MyApp) -> Result<Command<Msg>, String> {
        let profile_dir_name: String = make_profile_dir_name_valid(&self.profile_name);
        let profile_dir: PathBuf = app.home_dir.join("Profiles").join(profile_dir_name);
        fs::create_dir_all(&profile_dir)
            .map_err(|e| format!("Could not create profile directory: {e}"))?;

        fs::create_dir(profile_dir.join("Mods"))
            .map_err(|e| format!("Could not create profile mods directory: {e}"))?;

        let date_now: chrono::DateTime<chrono::Utc> = chrono::Utc::now();

        let profile = Profile {
            index: app.profiles.len(),
            name: self.profile_name.clone(),
            game_info: self.game_info.clone(),
            date_created: date_now.with_timezone(&chrono::Local),
            last_used: date_now.with_timezone(&chrono::Local),
            mods: vec![],
            icon: self.icon.clone(),
            path: profile_dir.clone(),
        };
        
        let config_file: PathBuf = profile_dir.join("profile.json");
        let date_string: String = date_now.to_string();
        let game_name: String = self.game_info.game_type.to_string()
            .ok_or("Game Type is somehow not set; this should have already been checked, though.".to_string())?;

        let config: serde_json::Value = serde_json::json!({
            "displayName": self.profile_name,
            "dateCreated": date_string,
            "lastUsed": date_string,
            "gameName": game_name,
            "gameVersion": [self.game_info.version.major, self.game_info.version.minor],
            "mods": [],
        });
        let config: String = serde_json::to_string_pretty(&config).expect("Could not convert (half literal) json to string");

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

