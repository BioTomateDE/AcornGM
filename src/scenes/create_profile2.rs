use iced::{alignment, Element};
use iced::widget::{container, column, text, row, button, TextInput};
use crate::{Msg, SceneMain, SceneType};
use crate::scenes::create_profile1::SceneCreateProfile;
use crate::scenes::homepage::SceneHomePage;

#[derive(Debug, Clone)]
pub enum MsgCreateProfile2 {
    BackToHomepage,
    StepBack,
    StepNext,
    EditDataPath(String),
}

impl SceneMain {
    pub fn update_create_profile2(&mut self, message: Msg) {
        let scene: &mut SceneCreateProfile = match &mut self.active_scene {
            SceneType::CreateProfile2(scene) => scene,
            _ => {
                println!("[ERROR @ SceneMain::update_create_profile2]  Could not extract scene: {:?}", self.active_scene);
                return;
            }
        };

        match message {
            Msg::CreateProfile2(MsgCreateProfile2::BackToHomepage) => {
                self.active_scene = SceneType::HomePage(SceneHomePage::default());
            },
            Msg::CreateProfile2(MsgCreateProfile2::StepBack) => {
                self.active_scene = SceneType::CreateProfile1(scene.clone());
            },
            // Msg::CreateProfile2(MsgCreateProfile2::Next) => {
            //     self.active_scene = SceneType::CreateProfile3(scene_create_profile.clone())
            // }
            Msg::CreateProfile2(MsgCreateProfile2::EditDataPath(profile_name)) => {
                scene.is_profile_name_valid = check_profile_name_valid(&profile_name)
            }
            _ => {},
        }
    }

    pub fn view_create_profile2(&self, scene_create_profile: &SceneCreateProfile) -> Element<Msg> {
        // let data_path_valid = text(
        //     if scene_create_profile.is_profile_name_valid {"Invalid Profile Name"} else {""}
        // ).size(12).color(self.color_text_red);

        let main_content = container(
            iced::widget::column![
                column![
                    // text("").size(10),
                    text("Create New Profile").size(22).color(self.color_text1),
                    text("").size(10),
                    // text("Recent Profiles").size(12).color(self.color_text2).align_x(alignment::Horizontal::Center),
                    text("GameMaker Data File").size(14).color(self.color_text2),
                    text("").size(10),
                    TextInput::new(
                        "/path/to/data.win",
                        &scene_create_profile.data_file_path
                    ).on_input(|string| Msg::CreateProfile2(MsgCreateProfile2::EditDataPath(string))),
                    text("").size(4),
                    // data_path_valid,
                ]
                .padding(20)
            ]
        ).align_x(alignment::Horizontal::Left);

        let button_bar = container(
            row![
                container(
                    row![
                        text("    ").size(10),
                        button("< Back").on_press(Msg::CreateProfile2(MsgCreateProfile2::StepBack)),
                        button("Cancel").on_press(Msg::CreateProfile2(MsgCreateProfile2::BackToHomepage)),
                    ]
                    .spacing(10)
                )
                .align_x(alignment::Horizontal::Right),
                text("                                                                  ").size(20),
                container(
                     row![
                        button("Next >").on_press(Msg::CreateProfile2(MsgCreateProfile2::StepNext)),
                        text("    ").size(10),
                    ]
                    .spacing(10)
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

    profile_name.len() > 100 ||
        profile_name.len() < 1
}

