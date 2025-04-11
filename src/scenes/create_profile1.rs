use std::path::PathBuf;
use std::time::Instant;
use iced::{alignment, Command, Element};
use iced::widget::{container, column, text, row, button, TextInput, Image};
use iced::widget::image::Handle;
use crate::{Msg, MyApp, SceneType};
use crate::scenes::homepage::SceneHomePage;
use crate::utility::GameInfo;
use crate::default_file_paths::get_default_image_prompt_path;

#[derive(Debug, Clone)]
pub enum MsgCreateProfile1 {
    BackToHomepage,
    StepNext,
    EditProfileName(String),
    EditProfileIcon,
}

#[derive(Debug, Clone)]
pub struct SceneCreateProfile {
    pub profile_name: String,
    pub is_profile_name_valid: bool,
    pub icon: Handle,
    pub data_file_path: String,
    pub game_info: GameInfo,
    pub game_name: String,      // used as a buffer for text input; represents .game_info(GameInfo::Other(string))
    pub game_version_str: String,
    pub is_game_version_valid: bool,
    pub currently_loading_data_file: bool,
}

impl MyApp {
    pub fn update_create_profile1(&mut self, message: Msg) -> Command<Msg> {
        let scene: &mut SceneCreateProfile = match &mut self.active_scene {
            SceneType::CreateProfile1(scene) => scene,
            _ => {
                println!("[ERROR @ create_profile1::update]  Could not extract scene: {:?}", self.active_scene);
                return Command::none();
            }
        };

        match message {
            Msg::CreateProfile1(MsgCreateProfile1::BackToHomepage) => {
                self.active_scene = SceneType::HomePage(SceneHomePage {});
            },
            Msg::CreateProfile1(MsgCreateProfile1::StepNext) => {
                if scene.is_profile_name_valid {
                    self.active_scene = SceneType::CreateProfile2(scene.clone())
                }
            }
            Msg::CreateProfile1(MsgCreateProfile1::EditProfileName(profile_name)) => {
                scene.is_profile_name_valid = check_profile_name_valid(&profile_name);
                scene.profile_name = profile_name;
            }
            Msg::CreateProfile1(MsgCreateProfile1::EditProfileIcon) => {
                let default_origin_path: PathBuf = get_default_image_prompt_path().unwrap_or_else(|error| {
                    println!("[WARN @ create_profile1::update]  Could not get default image prompt path: {error}");
                    self.current_working_dir.clone()
                });

                let image_path = native_dialog::FileDialog::new()
                    .set_location(&default_origin_path)
                    .add_filter("Image", &["png", "jpg", "jpeg", "webp", "gif"])
                    .show_open_single_file();
                let image_path = match image_path {
                    Ok(ok) => ok,
                    Err(error) => { println!("[WARN @ create_profile1::update]  Could not get path from file picker: {}", error); return Command::none(); }
                };
                let image_path: PathBuf = match image_path {
                    Some(ok) => ok,
                    None => { println!("[WARN @ create_profile1::update]  Path from file picker is empty"); return Command::none();}
                };
                if !image_path.is_file() {
                    println!("[WARN @ create_profile1::update]  Specified image path for icon doesn't exist: {}", image_path.display());
                    return Command::none()
                }
                scene.icon = Handle::from_path(image_path);

            },
            _ => {},
        }
        Command::none()
    }

    pub fn view_create_profile1(&self, scene_create_profile: &SceneCreateProfile) -> Element<Msg> {
        let scene: &SceneCreateProfile = match &self.active_scene {
            SceneType::CreateProfile1(scene) => scene,
            _ => {
                println!("[ERROR @ create_profile1::update]  Could not extract scene: {:?}", self.active_scene);
                return column![text("Error (look in logs)")].into()
            }
        };

        let profile_name_valid = text(
            if scene_create_profile.is_profile_name_valid {""} else {"Invalid Profile Name"}
        ).size(12).style(self.color_text_red);

        let icon: Image<Handle> = Image::new(scene.icon.clone());

        let main_content = container(
            iced::widget::column![
                column![
                    text("Create New Profile").size(22).style(self.color_text1),
                    text("").size(10),
                    text("Profile Name").size(14).style(self.color_text2),
                    text("").size(4),
                    TextInput::new(
                        "My Profile",
                        &scene_create_profile.profile_name
                    ).on_input(|string| Msg::CreateProfile1(MsgCreateProfile1::EditProfileName(string))),
                    text("").size(4),
                    profile_name_valid,
                    text("").size(10),
                    text("Profile Icon").size(14).style(self.color_text2),
                    text("").size(4),
                    button(icon.height(100)).on_press(Msg::CreateProfile1(MsgCreateProfile1::EditProfileIcon)),
                ]
                .padding(20)
            ]
        ).align_x(alignment::Horizontal::Left);

        let button_bar = container(
            row![
                container(
                    row![
                        text("    ").size(10),
                        button("Cancel").on_press(Msg::CreateProfile1(MsgCreateProfile1::BackToHomepage)),
                    ]
                    .spacing(10)
                ),
                text("                                                                    ").size(20),
                container(
                     row![
                        button("Next >").on_press(Msg::CreateProfile1(MsgCreateProfile1::StepNext)),
                        text("    ").size(10),
                    ]
                    .spacing(10)
                )
            ]
        )
            // .align_x(alignment::Horizontal::Right)
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


pub fn check_profile_name_valid(profile_name: &str) -> bool {
    let profile_name: &str = profile_name.trim();

    profile_name.len() < 100 &&
    profile_name.len() > 0
}

