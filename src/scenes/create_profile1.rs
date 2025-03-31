use iced::{alignment, Element};
use iced::widget::{container, column, text, row, button, TextInput};
use crate::{GameType, Msg, SceneMain, SceneType};
use crate::scenes::homepage::SceneHomePage;

#[derive(Debug, Clone)]
pub enum MsgCreateProfile1 {
    BackToHomepage,
    Next,
    EditProfileName(String),
}

#[derive(Default, Debug, Clone)]
pub struct SceneCreateProfile {
    pub profile_name: String,
    pub is_profile_name_valid: bool,
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
            Msg::CreateProfile1(MsgCreateProfile1::Next) => {
                self.active_scene = SceneType::CreateProfile2(scene.clone())
            }
            Msg::CreateProfile1(MsgCreateProfile1::EditProfileName(profile_name)) => {
                scene.is_profile_name_valid = check_profile_name_valid(&profile_name);
                scene.profile_name = profile_name;
            }
            _ => {},
        }
    }

    pub fn view_create_profile1(&self, scene_create_profile: &SceneCreateProfile) -> Element<Msg> {
        let profile_name_valid = text(
            if scene_create_profile.is_profile_name_valid {""} else {"Invalid Profile Name"}
        ).size(12).color(self.color_text_red);

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
                        button("Next >").on_press(Msg::CreateProfile1(MsgCreateProfile1::Next)),
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

