use iced::{alignment, Command, Element};
use iced::widget::{container, row, column, text, button};
use crate::{Msg, MyApp};
use crate::scenes::homepage::Profile;


#[derive(Debug, Clone)]
pub enum MsgViewProfile {
    BackToHomepage,
}

#[derive(Default, Debug, Clone)]
pub struct SceneViewProfile {
    pub profile: Profile,
}

impl MyApp {
    pub fn update_view_profile(&mut self, _message: Msg) -> Command<Msg> {
        Command::none()
    }

    pub fn view_view_profile(&self, _scene: &SceneViewProfile) -> Element<Msg> {
        let main_content = container(
            iced::widget::column![
                column![
                    text("Sample text").size(22).style(self.color_text1),
                    text("").size(10),
                    text("GameMaker Data File").size(14).style(self.color_text2),
                    ].spacing(69),
                ]
                .padding(20)
        ).align_x(alignment::Horizontal::Left);

        let button_bar = container(
            row![
                container(
                    row![
                        text("    ").size(10),
                        button("Home").on_press(Msg::ViewProfile(MsgViewProfile::BackToHomepage)),
                    ]
                    .spacing(10)
                ),
                text("                                                                    ").size(20),
                container(
                     row![
                        button("Next >").on_press(Msg::ViewProfile(MsgViewProfile::BackToHomepage)),
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

