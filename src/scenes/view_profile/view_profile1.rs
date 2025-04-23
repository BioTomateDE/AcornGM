use std::sync::{Arc, Mutex};
use crate::scenes::view_profile::{AcornMod, SceneViewProfile};
use iced::{Color, Command, Element};
use iced::advanced::image::Handle;
use iced::widget::{container, row, column, text, button, Image, Container, scrollable};
use iced::widget::container::Appearance;
use log::error;
use crate::{Msg, MyApp, Scene, SceneType, COLOR_TEXT1, COLOR_TEXT2, WINDOW_SIZE_NORMAL};
use crate::scenes::browser::MsgBrowser;
use crate::scenes::homepage::{create_divider, list_style, SceneHomePage};
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
                    text("").size(10),
                    row![
                        text("   ").size(20),
                        icon.width(50),
                        text("    ").size(10),
                        column![
                            row![
                                text(&self.name).size(16).style(*COLOR_TEXT1),
                                text("      ").size(10),
                                column![
                                    text("").size(4),
                                    text(format!("v{}", self.mod_version)).size(12).style(*COLOR_TEXT2),
                                ],
                            ],
                            text("").size(6),
                            text(format!("by {}", self.author_name)).size(13).style(*COLOR_TEXT1),
                        ]
                    ]
                ]
            )
                .style(iced::theme::Button::Custom(Box::new(TransparentButton)))
                .on_press(Msg::ViewProfile(MsgViewProfile::ViewModDetails(self.clone())))
        )
            .width(700)
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
                app.active_scene = SceneType::HomePage(((/*trt*/SceneHomePage)));
                return iced::window::resize(app.flags.main_window_id, WINDOW_SIZE_NORMAL)
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
            .width(380)
            .style(list_style);

        let my_font = iced::Font {
            family: iced::font::Family::Serif,
            weight: Default::default(),
            stretch: Default::default(),
            style: Default::default(),
        };

        let mods_content = container(
            iced::widget::column![
                column![
                    text("Mod Manager").size(22).style(*COLOR_TEXT1),
                    text("").size(10),
                    text("Mods").size(14).style(*COLOR_TEXT2),
                    text("").size(6),
                    row![
                        scrollable(mods).height(800),
                        text("    ").size(10),
                        column![
                            text("").size(30),
                            button(text("^").font(my_font)).on_press(Msg::ViewProfile(MsgViewProfile::MoveModPriorityUp)),
                            button(text("X").font(my_font)).on_press(Msg::ViewProfile(MsgViewProfile::ToggleModActive)),
                            button(text("v").font(my_font)).on_press(Msg::ViewProfile(MsgViewProfile::MoveModPriorityDown)),
                        ].spacing(9)
                    ],
                ],
            ].padding(20)
        ).width(450);

        let button_bar = container(
            row![
                container(
                    row![
                        text("    ").size(10),
                        button("Home").on_press(Msg::ViewProfile(MsgViewProfile::BackToHomepage)),
                    ]
                    .spacing(10)
                ),
                text("                                           ").size(20),
                container(
                     row![
                        button("Launch Game").on_press(Msg::ViewProfile(MsgViewProfile::LaunchGame)),
                        text("    ").size(10),
                    ]
                    .spacing(10)
                )
            ]
        )
            .width(900);


        let browser_content: Element<Msg> = self.browser.view();
        let mod_details_content: Element<Msg> = self.mod_details.view();

        container(
            column![
                row![
                    column![mods_content].height(750),
                    text("       ").size(10),
                    column![
                        column![browser_content].height(350),
                        text("").size(10),
                        column![mod_details_content].height(350),
                    ]
                ],
                button_bar
            ]
        )
            .into()
    }
}

