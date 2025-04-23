use std::sync::{Arc, Mutex};
use crate::utility::ACORN_BASE_URL;
use iced::{alignment, Command, Element};
use iced::widget::{button, column, container, row, scrollable, text, Container};
use log::error;
use crate::{Msg, MyApp, Scene, SceneType, COLOR_TEXT1, COLOR_TEXT2, WINDOW_SIZE_VIEW_PROFILE};
use crate::scenes::browser::ModBrowser;
use crate::scenes::create_profile::SceneCreateProfile;
use crate::scenes::homepage::{create_divider, list_style, MsgHomePage, SceneHomePage};
use crate::scenes::login::SceneLogin;
use crate::scenes::view_profile::SceneViewProfile;
use crate::utility::{get_default_icon_image, GameInfo};

impl Scene for SceneHomePage {
    fn update(&mut self, app: &mut MyApp, message: Msg) -> Command<Msg> {
        let message = match message {
            Msg::HomePage(msg) => msg,
            other => {
                error!("Invalid message type {other:?}");
                return Command::none()
            }
        };

        match message {
            MsgHomePage::CreateProfile => {
                app.active_scene = SceneType::CreateProfile(SceneCreateProfile {
                    stage: 1,
                    profile_name: "My Profile".to_string(),
                    is_profile_name_valid: true,
                    icon: get_default_icon_image(&app.app_root),
                    data_file_path: "".to_string(),
                    game_info: GameInfo::default(),
                    game_name: "".to_string(),
                    game_version_str: "".to_string(),
                    is_game_version_valid: true,        // to hide error when no data file is loaded
                });
            },

            MsgHomePage::LoadProfile(index) => {
                if let Some(profile) = app.profiles.get(index) {
                    app.active_scene = SceneType::ViewProfile(SceneViewProfile {
                        mods: vec![],
                        profile: profile.clone(),
                        browser: ModBrowser {
                            search_query: "".to_string(),
                            use_regex: false,
                            results: vec![],
                            show_only_compatible: true,
                        },
                        mod_details: Default::default(),
                    });
                }
                return iced::window::resize(app.flags.main_window_id, WINDOW_SIZE_VIEW_PROFILE)
            },

            MsgHomePage::Login => {
                let temp_login_token: String = uuid::Uuid::new_v4().to_string();
                let url: String = format!("{ACORN_BASE_URL}/goto_discord_auth?temp_login_token={}", temp_login_token);

                app.active_scene = SceneType::Login(SceneLogin {
                    temp_login_token,
                    url,
                });
            },
        }
        Command::none()
    }
    fn view<'a>(&'a self, app: &'a MyApp) -> Element<'a, Msg> {
        let mut profiles: Vec<Element<Msg>> = Vec::new();
        for (_i, profile) in app.profiles.iter().enumerate() {
            profiles.push(profile.view(*COLOR_TEXT1, *COLOR_TEXT2));
            profiles.push(create_divider())
        }
        let profiles: Container<Msg> = container(column(profiles).spacing(5)).style(list_style);

        let main_content = container(
            iced::widget::column![
                    column![
                        text("").size(10),
                        text("AcornGM").size(28).style(*COLOR_TEXT1),
                        text("").size(6),
                        text("Recent Profiles").size(14).style(*COLOR_TEXT2).horizontal_alignment(alignment::Horizontal::Center),
                        text("").size(6),
                        scrollable(profiles).height(500),
                    ]
                    .padding(20)
                ]
        ).align_x(alignment::Horizontal::Left);

        let button_bar = container(
            row![
                    container(
                        row![
                            text("    ").size(10),
                            button("Login").on_press(Msg::HomePage(MsgHomePage::Login)),
                        ]
                        .spacing(10)
                    ),
                    text("                                                               ").size(19),
                    container(
                         row![
                            button("Create Profile").on_press(Msg::HomePage(MsgHomePage::CreateProfile)),
                            text("    ").size(10),
                        ]
                        .spacing(10)
                    )
                ]
        )
            .width(900);

        container(
            iced::widget::column![
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
