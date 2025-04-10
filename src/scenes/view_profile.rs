use iced::{Color, Command, Element};
use iced::advanced::image::Handle;
use iced::widget::{container, row, column, text, button, Image, Container, scrollable};
use iced::widget::container::Appearance;
use crate::{Msg, MyApp, SceneType, WINDOW_SIZE_NORMAL};
use crate::scenes::browser::{ModBrowser, MsgBrowser};
use crate::scenes::homepage::{create_divider, list_style, Profile};
use crate::utility::{GameType, PlatformType, TransparentButton, Version};

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

#[derive(Default, Debug, Clone)]
pub struct SceneViewProfile {
    pub profile: Profile,
    pub mods: Vec<AcornMod>,
    pub browser: ModBrowser,
}


#[derive(Debug, Clone)]
pub struct AcornMod {
    pub name: String,
    pub icon: Handle,
    pub author_name: String,
    pub mod_version: Version,
    pub date_created: chrono::DateTime<chrono::Local>,
    pub date_last_updated: chrono::DateTime<chrono::Local>,
    pub supported_games: Vec<GameType>,
    pub supported_game_versions: Vec<Version>,
    pub supported_platforms: Vec<PlatformType>,
    pub description: String,
}
impl AcornMod {
    pub fn view(&self, color_text1: Color, color_text2: Color) -> Element<Msg> {
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
                                text(&self.name).size(16).style(color_text1),
                                text("      ").size(10),
                                column![
                                    text("").size(4),
                                    text(format!("v{}", self.mod_version)).size(12).style(color_text2),
                                ],
                            ],
                            text("").size(6),
                            text(format!("by {}", self.author_name)).size(13).style(color_text1),
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
        // border: iced::Border {
        //     color: Color::from_rgb8(21, 22, 24),
        //     width: 3.0,
        //     radius: iced::border::Radius::from([9.8, 9.9, 9.9, 9.9]),
        // },
        border: iced::Border::default(),
        shadow: Default::default(),
    }
}


impl MyApp {
    pub fn update_view_profile(&mut self, message: Msg) -> Command<Msg> {
        let scene: &mut SceneViewProfile = match &mut self.active_scene {
            SceneType::ViewProfile(scene) => scene,
            _ => {
                println!("[ERROR @ view_profile::update]  Could not extract scene: {:?}", self.active_scene);
                return Command::none();
            }
        };

        match message {
            Msg::ViewProfile(MsgViewProfile::BackToHomepage) => {
                self.active_scene = SceneType::HomePage(crate::scenes::homepage::SceneHomePage {});
                return iced::window::resize(self.flags.main_window_id, WINDOW_SIZE_NORMAL)
            },

            Msg::ViewProfile(MsgViewProfile::ViewModDetails(acorn_mod)) => {
                // stub
            },

            Msg::ViewProfile(MsgViewProfile::LaunchGame) => {
                // stub
            },

            Msg::ViewProfile(MsgViewProfile::Browser(msg)) => {
                return scene.browser.update(msg)
            }

            _ => {},
        }
        Command::none()
    }

    pub fn view_view_profile<'a>(&self, scene: &'a SceneViewProfile) -> Element<'a, Msg> {
        let mut mods: Vec<Element<Msg>> = Vec::new();
        for (_i, acorn_mod) in scene.mods.iter().enumerate() {
            mods.push(acorn_mod.view(self.color_text1, self.color_text2));
            mods.push(create_divider())
        }
        let mods: Container<Msg> = container(column(mods).spacing(5))
            .width(380)
            .style(list_style);

        let my_font = iced::Font {
            family: iced::font::Family::Serif,
            // family: iced::font::Family::Name("wingdings"),
            weight: Default::default(),
            stretch: Default::default(),
            style: Default::default(),
        };

        let mods_content = container(
            iced::widget::column![
                column![
                    text("Mod Manager").size(22).style(self.color_text1),
                    text("").size(10),
                    text("Mods").size(14).style(self.color_text2),
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

        let browser_content = scene.browser.view(self.color_text1, self.color_text2);

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

        container(
            column![
                row![
                    column![mods_content].height(750),
                    text("       ").size(10),
                    column![browser_content].height(350),
                ],
                button_bar
            ]
        )
            .into()
    }
}

