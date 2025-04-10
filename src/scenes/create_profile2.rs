use std::fs;
use std::path::PathBuf;
use iced::{alignment, Command, Element};
use iced::widget::{container, column, text, row, button, TextInput};
use sha256;
use crate::{Msg, MyApp, SceneType};
use crate::default_file_paths::{get_default_data_file_dir, show_msgbox};
use crate::scenes::create_profile1::SceneCreateProfile;
use crate::scenes::homepage::SceneHomePage;
use crate::utility::{GameInfo, GameType, Version};

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

impl MyApp {
    pub fn update_create_profile2(&mut self, message: Msg) -> Command<Msg> {
        let scene: &mut SceneCreateProfile = match &mut self.active_scene {
            SceneType::CreateProfile2(scene) => scene,
            _ => {
                println!("[ERROR @ create_profile2::update]  Could not extract scene: {:?}", self.active_scene);
                return Command::none();
            }
        };

        match message {
            Msg::CreateProfile2(MsgCreateProfile2::BackToHomepage) => {
                self.active_scene = SceneType::HomePage(SceneHomePage {});
            },

            Msg::CreateProfile2(MsgCreateProfile2::StepBack) => {
                self.active_scene = SceneType::CreateProfile1(scene.clone());
            },

            Msg::CreateProfile2(MsgCreateProfile2::StepNext) => {
                if !scene.is_profile_name_valid { return Command::none() }
                match scene.game_info.game_type {
                    GameType::Unset => return Command::none(),
                    _ => {}
                }

                let profile_name: String = make_profile_dir_name_valid(&scene.profile_name);
                let profile_dir: PathBuf = self.home_dir.join(format!("./Profiles/{}", profile_name));
                match fs::create_dir_all(&profile_dir) {
                    Ok(_) => {},
                    Err(error) => show_msgbox(
                        "Error creating AcornGM profile",
                        &format!("Could not create profile directory: {error}"
                        ))
                };

                // create config file
                let config_file: PathBuf = profile_dir.join("./profile.json");
                let date: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
                let date: String = date.to_string();
                let game_name: String = match &scene.game_info.game_type {
                    GameType::Undertale => "Undertale".to_string(),
                    GameType::Deltarune => "Deltarune".to_string(),
                    GameType::Other(name) => name.clone(),
                    GameType::Unset => return Command::none(),
                };

                let config: serde_json::Value = serde_json::json!({
                    "displayName": scene.profile_name,
                    "dateCreated": date,
                    "lastUsed": date,
                    "gameName": game_name,
                    "gameVersion": [scene.game_info.version.major, scene.game_info.version.minor],
                    "mods": [],
                });
                let config: String = serde_json::to_string_pretty(&config).unwrap();

                match fs::write(config_file, config) {
                    Ok(_) => {},
                    Err(error) => show_msgbox(
                        "Error creating AcornGM profile",
                        &format!("Could not create profile config file: {error}"
                        ))
                };

                // create icon file
                let icon_file: PathBuf = profile_dir.join("./icon.png");
                match scene.icon.save(icon_file) {
                    Ok(_) => {},
                    Err(error) => show_msgbox(
                        "Error creating AcornGM profile",
                        &format!("Could not create profile icon file: {error}"
                        ))
                };

                // copy data win
                let data_file: PathBuf = profile_dir.join("./data.win");
                match fs::copy(&scene.data_file_path, data_file) {
                    Ok(_) => {},
                    Err(error) => show_msgbox(
                        "Error creating AcornGM profile",
                        &format!("Could not copy data file: {error}"
                        ))
                };

                // TODO replace with profile scene
                self.active_scene = SceneType::HomePage(SceneHomePage {});
            }

            Msg::CreateProfile2(MsgCreateProfile2::EditDataPath(data_file_path)) => {
                scene.data_file_path = data_file_path;
            },

            Msg::CreateProfile2(MsgCreateProfile2::SubmitDataPath) => {
                self.detect_game();
            },

            Msg::CreateProfile2(MsgCreateProfile2::PickDataPath) => {
                let default_data_dir: PathBuf = match get_default_data_file_dir() {
                    Ok(path) => path,
                    Err(error) => {
                        println!("[WARN @ create_profile2::update]  Could not get default data file path: {error}"); return Command::none();
                    }
                };
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
                scene.data_file_path = data_path.to_string();
                self.detect_game();
            },

            Msg::CreateProfile2(MsgCreateProfile2::EditGameName(name)) => {
                match &scene.game_info.game_type {
                    GameType::Other(_) => {
                        scene.game_name = name.clone();
                        scene.game_info.game_type = GameType::Other(name);
                    },
                    _ => {},
                }
            },

            Msg::CreateProfile2(MsgCreateProfile2::EditGameVersion(version_str)) => {
                // ignore if no data file loaded or if version was automatically detected
                if let GameType::Other(_) = scene.game_info.game_type {
                    return Command::none()
                }
                let version: Version = match version_str.parse() {
                    Ok(ver) => ver,
                    Err(_) => {
                        scene.is_game_version_valid = false;
                        return Command::none();
                    }
                };
                match &scene.game_info.game_type {
                    GameType::Other(_) => scene.game_info.version = version,
                    _ => {},
                }
                scene.is_game_version_valid = true;
            },

            _ => {},
        }
        Command::none()
    }

    pub fn view_create_profile2(&self, scene: &SceneCreateProfile) -> Element<Msg> {
        let game_version_valid = text(
            if scene.is_game_version_valid {""} else {"Invalid Version (example for valid version: 1.63)"}
        ).size(12).style(self.color_text_red);

        let auto_detected = text(
            match scene.game_info.game_type {
                GameType::Unset => "",
                GameType::Other(_) => "Could not determine game version!\nIf your game is Undertale or Deltarune, please make sure it is not modified!",
                _ => "Game Name and Version automatically detected!",
            }
        );

        let main_content = container(
            iced::widget::column![
                column![
                    text("Create New Profile").size(22).style(self.color_text1),
                    text("").size(10),
                    text("GameMaker Data File").size(14).style(self.color_text2),
                    text("").size(4),
                    row![
                        TextInput::new(
                            "/path/to/data.win",
                            &scene.data_file_path
                        )
                            .on_input(|string| Msg::CreateProfile2(MsgCreateProfile2::EditDataPath(string)))
                            .on_submit(Msg::CreateProfile2(MsgCreateProfile2::SubmitDataPath)),
                        button("Pick File").on_press(Msg::CreateProfile2(MsgCreateProfile2::PickDataPath)),
                    ].spacing(10),
                    text("").size(16),
                    row![
                        column![
                            text("Game Name").size(14).style(self.color_text2),
                            text("").size(4),
                            TextInput::new("Game", &scene.game_name)
                                .on_input(|string| Msg::CreateProfile2(MsgCreateProfile2::EditGameName(string)))
                        ],
                        column![
                            text("Game Version").size(14).style(self.color_text2),
                            text("").size(4),
                            TextInput::new("Version", &scene.game_version_str)
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
        let scene: &mut SceneCreateProfile = match &mut self.active_scene {
            SceneType::CreateProfile2(scene) => scene,
            _ => {
                println!("[ERROR @ create_profile2::detect_game]  Could not extract scene: {:?}", self.active_scene);
                return;
            }
        };

        match detect_game_and_version(&scene.data_file_path) {
            Ok(game_info) => {
                scene.game_name = match &game_info.game_type {
                    GameType::Other(name) => name.clone(),
                    GameType::Undertale => "Undertale".to_string(),
                    GameType::Deltarune => "Deltarune".to_string(),
                    GameType::Unset => "".to_string(),
                };
                scene.game_version_str = game_info.version.to_string();
                scene.game_info = game_info;
            },
            Err(_) => {},
        };
    }
}


fn detect_game_and_version(data_file_path: &str) -> Result<GameInfo, String> {
    let bytes: Vec<u8> = match fs::read(data_file_path) {
        Ok(bytes) => bytes,
        Err(error) => return Err(format!("Could not read data file: {error}")),
    };
    let hash: String = sha256::digest(bytes);
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


pub fn check_profile_name_valid(profile_name: &str) -> bool {
    let profile_name: &str = profile_name.trim();

    profile_name.len() > 100 ||
        profile_name.len() < 1
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

    if name.len() < 1 || name.ends_with('.') || BANNED_NAMES.contains(&name.to_uppercase().as_str()) {
        name = uuid::Uuid::new_v4().hyphenated().to_string();
    }
    name
}
