use iced::{alignment, Element};
use iced::widget::{container, column, text, row, button, TextInput};
use crate::{GameType, Msg, SceneMain, SceneType};
use crate::scenes::homepage::SceneHomePage;
use crate::utility::{get_current_working_directory, get_default_image_prompt_path, img_to_iced};
use image;
use image::GenericImageView;

#[derive(Debug, Clone)]
pub enum MsgCreateProfile1 {
    BackToHomepage,
    StepNext,
    EditProfileName(String),
    EditProfileIcon,
}

#[derive(Default, Debug, Clone)]
pub struct SceneCreateProfile {
    pub profile_name: String,
    pub is_profile_name_valid: bool,
    pub icon: image::DynamicImage,
    pub profile_path: String,
    pub data_file_path: String,
    pub game_type: GameType,
}

impl SceneMain {
    pub fn update_create_profile1(&mut self, message: Msg) {
        let scene: &mut SceneCreateProfile = match &mut self.active_scene {
            SceneType::CreateProfile1(scene) => scene,
            _ => {
                println!("[ERROR @ SceneMain::update_create_profile1]  Could not extract scene: {:?}", self.active_scene);
                return;
            }
        };

        match message {
            Msg::CreateProfile1(MsgCreateProfile1::BackToHomepage) => {
                self.active_scene = SceneType::HomePage(SceneHomePage::default());
            },
            Msg::CreateProfile1(MsgCreateProfile1::StepNext) => {
                self.active_scene = SceneType::CreateProfile2(scene.clone())
            }
            Msg::CreateProfile1(MsgCreateProfile1::EditProfileName(profile_name)) => {
                scene.is_profile_name_valid = check_profile_name_valid(&profile_name);
                scene.profile_name = profile_name;
            }
            Msg::CreateProfile1(MsgCreateProfile1::EditProfileIcon) => {
                let default_origin_path: String = get_default_image_prompt_path().unwrap_or_else(|error| {
                    println!("[WARN @ SceneMain::update_create_profile1]  Could not get default image prompt path: {error}");
                    get_current_working_directory().unwrap_or_else(|| "".to_string())
                });

                let image_path = native_dialog::FileDialog::new()
                    .set_location(&default_origin_path)
                    .add_filter("PNG Image", &["png"])
                    .add_filter("JPEG Image", &["jpg", "jpeg"])
                    .show_open_single_file();
                let image_path = image_path.unwrap_or_else(|error| {
                    println!("[WARN @ SceneMain::update_create_profile1]  Could not get path from file picker: {}", error);
                    None
                });
                let image_path = image_path.unwrap_or_else(|| {
                    println!("[WARN @ SceneMain::update_create_profile1]  Path from file picker is empty");
                    return Default::default();
                });

                let img = match image::open(image_path) {
                    Ok(img) => img,
                    Err(error) => {
                        println!("[WARN @ SceneMain::update_create_profile1]  Failed to parse image: {}", error);
                        return;
                    }
                };
                scene.icon = img;

            }
            _ => {},
        }
    }

    pub fn view_create_profile1(&self, scene_create_profile: &SceneCreateProfile) -> Element<Msg> {
        let scene: &SceneCreateProfile = match &self.active_scene {
            SceneType::CreateProfile1(scene) => scene,
            _ => {
                println!("[ERROR @ SceneMain::update_create_profile1]  Could not extract scene: {:?}", self.active_scene);
                return column![text("Error (look in logs)")].into()
            }
        };

        let profile_name_valid = text(
            if scene_create_profile.is_profile_name_valid {""} else {"Invalid Profile Name"}
        ).size(12).color(self.color_text_red);

        // scene.icon.write_to()
        // let icon = iced::widget::image::Handle::from_rgba(scene.icon.width(), scene.icon.height(), scene.icon.);
        // let icon = iced::widget::image(icon);
        let icon = img_to_iced(&scene.icon);

        let main_content = container(
            iced::widget::column![
                column![
                    // text("").size(10),
                    text("Create New Profile").size(22).color(self.color_text1),
                    text("").size(10),
                    // text("Recent Profiles").size(12).color(self.color_text2).align_x(alignment::Horizontal::Center),
                    text("Profile Name").size(14).color(self.color_text2),
                    text("").size(10),
                    TextInput::new(
                        &scene_create_profile.profile_name,
                        &scene_create_profile.profile_name
                    ).on_input(|string| Msg::CreateProfile1(MsgCreateProfile1::EditProfileName(string))),
                    text("").size(4),
                    profile_name_valid,
                    text("").size(10),
                    text("Profile Icon").size(14).color(self.color_text2),
                    text("").size(4),
                    button(icon.height(100)).on_press(Msg::CreateProfile1(MsgCreateProfile1::EditProfileIcon)),
                    // button(iced::widget::image(scene.icon).height(100)).on_press(Msg::CreateProfile1(MsgCreateProfile1::EditProfileIcon)),
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
                )
                .align_x(alignment::Horizontal::Right),
                text("                                                                  ").size(20),
                container(
                     row![
                        button("Next >").on_press(Msg::CreateProfile1(MsgCreateProfile1::StepNext)),
                        text("    ").size(10),
                    ]
                )
                .align_x(alignment::Horizontal::Left)
            ]
        )
            .align_x(alignment::Horizontal::Right)
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

