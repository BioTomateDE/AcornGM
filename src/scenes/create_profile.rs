use iced::{alignment, Color, Element};
use iced::widget::{container, column, Column, text, scrollable, row, button};
use crate::{GameType, Msg, SceneMain, SceneType};


#[derive(Debug, Clone)]
pub enum MsgCreateProfile {
    BackToHomepage,
    Next,
}

#[derive(Default, Debug, Clone)]
pub struct SceneCreateProfile {
    pub profile_name: String,
    pub profile_path: String,
    pub data_file_path: String,
    pub game_type: GameType,
}

impl SceneMain {
    pub fn update_create_profile(&mut self, scene_create_profile: &SceneCreateProfile, message: Msg) -> SceneType {
        let mut scene: SceneType = std::mem::take(&mut self.active_scene);

        scene
    }

    pub fn view_create_profile(&self, scene_create_profile: &SceneCreateProfile) -> Element<Msg> {
        let main_content = container(
            iced::widget::column![
                column![
                    text("").size(10),
                    text("GMAcorn").size(28).color(self.color_text1),
                    text("").size(6),
                    text("Recent Profiles").size(12).color(self.color_text2).align_x(alignment::Horizontal::Center),
                    // text("").size(18),
                ]
                .padding(20)
            ]
        ).align_x(alignment::Horizontal::Left);

        let button_bar = container(
            row![
                container(
                    row![
                        text("    ").size(10),
                        button("Cancel").on_press(Msg::CreateProfile(MsgCreateProfile::BackToHomepage)),
                    ]
                )
                .align_x(alignment::Horizontal::Right),
                text("                                                                  ").size(20),
                container(
                     row![
                        button("Next >").on_press(Msg::CreateProfile(MsgCreateProfile::Next)),
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

