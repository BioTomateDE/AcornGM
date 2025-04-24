use crate::scenes::view_profile::{AcornMod, SceneViewProfile};
use iced::{Color, Command, Element, Length};
use iced::advanced::image::Handle;
use iced::widget::{container, row, column, text, button, Image, Container, scrollable, Space};
use iced::widget::container::Appearance;
use log::error;
use crate::{Msg, MyApp, Scene, SceneType, COLOR_TEXT1, COLOR_TEXT2, WINDOW_SIZE_NORMAL};
use crate::scenes::browser::MsgBrowser;
use crate::scenes::homepage::{SceneHomePage};
use crate::ui_templates::{create_divider, generate_button_bar, list_style};
use crate::utility::TransparentButton;

#[derive(Debug, Clone)]
pub enum MsgViewProfile {
    BackToHomepage,
    MoveModPriorityUp,
    ToggleModActive,
    MoveModPriorityDown,
    ViewModDetails(AcornMod),
    LaunchGame,
    Browser(MsgBrowser),
}


impl AcornMod {
    pub fn view(&self) -> Element<Msg> {
        let icon: Image<Handle> = Image::new(self.icon.clone());

        container(
            button(
                column![
                    Space::with_height(8.0),
                    row![
                        Space::with_width(8.0),
                        icon.width(50),
                        Space::with_width(12.0),
                        column![
                            row![
                                text(&self.name).size(16).style(*COLOR_TEXT1),
                                Space::with_width(8.0),
                                column![
                                    Space::with_height(4.0),
                                    text(format!("v{}", self.mod_version)).size(12).style(*COLOR_TEXT2),
                                ],
                            ],
                            Space::with_height(5.0),
                            text(format!("by {}", self.author_name)).size(13).style(*COLOR_TEXT1),
                        ]
                    ]
                ]
            )
                .style(iced::theme::Button::Custom(Box::new(TransparentButton)))
                .on_press(Msg::ViewProfile(MsgViewProfile::ViewModDetails(self.clone())))
        )
            .style(mod_item_style)
            .height(80)
            .into()
    }
}


fn mod_item_style(_theme: &iced::Theme) -> Appearance {
    Appearance {
        text_color: None,
        background: Some(iced::Background::Color(Color::from_rgb8(31, 32, 34))),
        border: iced::Border::default(),
        shadow: Default::default(),
    }
}


impl Scene for SceneViewProfile {
    fn update(&mut self, app: &mut MyApp, message: Msg) -> Command<Msg> {
        let message: MsgViewProfile = match message {
            Msg::ViewProfile(msg) => msg,
            other => {
                error!("Invalid message type {other:?}");
                return Command::none()
            }
        };

        match message {
            MsgViewProfile::BackToHomepage => {
                app.active_scene = SceneType::HomePage(SceneHomePage);
                return iced::window::resize(app.main_window_id, WINDOW_SIZE_NORMAL)
            }

            MsgViewProfile::ViewModDetails(acorn_mod) => {
                self.mod_details.acorn_mod = Some(acorn_mod);
            }

            MsgViewProfile::Browser(msg) => {
                return self.browser.update(msg)
            }

            MsgViewProfile::MoveModPriorityUp => {}

            MsgViewProfile::MoveModPriorityDown => {}

            MsgViewProfile::ToggleModActive => {}

            MsgViewProfile::LaunchGame => {}
        }
        Command::none()
    }

    fn view<'a>(&'a self, _app: &'a MyApp) -> Element<'a, Msg> {
        let mut mods: Vec<Element<Msg>> = Vec::new();
        for acorn_mod in &self.mods {
            mods.push(acorn_mod.view());
            mods.push(create_divider())
        }
        let mods: Container<Msg> = container(column(mods).spacing(5))
            .width(Length::Fill)
            .style(list_style);

        let my_font = iced::Font {
            family: iced::font::Family::Serif,
            weight: Default::default(),
            stretch: Default::default(),
            style: Default::default(),
        };

        let mods_content = container(
            column![
                Space::with_height(8.0),
                text("Mod Manager").size(22).style(*COLOR_TEXT1),
                Space::with_height(4.0),
                text("Mods").size(14).style(*COLOR_TEXT2),
                Space::with_height(6.0),
                row![
                    column![
                        button(text("^").font(my_font)).on_press(Msg::ViewProfile(MsgViewProfile::MoveModPriorityUp)),
                        button(text("X").font(my_font)).on_press(Msg::ViewProfile(MsgViewProfile::ToggleModActive)),
                        button(text("v").font(my_font)).on_press(Msg::ViewProfile(MsgViewProfile::MoveModPriorityDown)),
                    ].spacing(9),
                    Space::with_width(8.0),
                    scrollable(mods).height(800),
                ],
            ],
        ).width(Length::Fill);

        let button_bar = generate_button_bar(vec![
            button("Home").on_press(Msg::ViewProfile(MsgViewProfile::BackToHomepage)).into(),
        ], vec![
            button("Launch Game").on_press(Msg::ViewProfile(MsgViewProfile::LaunchGame)).into(),
        ]);

        let browser_content: Element<Msg> = self.browser.view();
        let mod_details_content: Element<Msg> = self.mod_details.view();

        container(
            column![
                row![
                    Space::with_width(4.0),
                    column![mods_content].height(750).width(Length::FillPortion(1)),
                    Space::with_width(12.0),
                    column![
                        column![browser_content].height(350).padding(5),
                        Space::with_height(8.0),
                        column![mod_details_content].height(350),
                    ].width(Length::FillPortion(1))
                ]
                .height(Length::Fill),
                button_bar.padding(5)
            ]
        )
            .into()
    }
}

