use crate::utility::ACORN_BASE_URL;
use iced::{alignment, Command, Element, Length};
use iced::widget::{button, column, container, row, scrollable, text, Container, Space};
use iced::widget::image::Handle;
use crate::{Msg, MyApp, Scene, SceneType, COLOR_TEXT1, COLOR_TEXT2, WINDOW_SIZE_VIEW_PROFILE};
use crate::resources::DEFAULT_PROFILE_ICON;
use crate::scenes::create_profile::SceneCreateProfile;
use crate::scenes::homepage::{update_profile_config, MsgHomePage, SceneHomePage};
use crate::scenes::login::{generate_token, SceneLogin};
use crate::scenes::view_profile::SceneViewProfile;
use crate::ui_templates::{create_divider, generate_button_bar, list_style};
use crate::utility::GameInfo;

impl Scene for SceneHomePage {
    fn update(&mut self, app: &mut MyApp, message: Msg) -> Result<Command<Msg>, String> {
        let message: MsgHomePage = match message {
            Msg::HomePage(msg) => msg,
            other => return Err(format!("Invalid message type {other:?} for HomePage")),
        };

        match message {
            MsgHomePage::CreateProfile => {
                app.active_scene = SceneType::CreateProfile(SceneCreateProfile {
                    stage: 1,
                    profile_name: "My Profile".to_string(),
                    is_profile_name_valid: true,
                    icon: Handle::from_memory(DEFAULT_PROFILE_ICON),
                    data_file_path: "".to_string(),
                    game_info: GameInfo::default(),
                    game_version_str: "".to_string(),
                    game_auto_detected: false,
                    is_game_version_valid: true,        // to hide error when no data file is loaded
                    is_file_picker_open: false,
                });
            },

            MsgHomePage::LoadProfile(index) => {
                let Some(profile) = app.profiles.get_mut(index) else {
                    return Err(format!("Failed to open profile with index {} in profile list with length {}", index, app.profiles.len()))
                };

                // update last used timestamp and save config
                profile.last_used = chrono::Local::now();
                update_profile_config(profile)
                    .unwrap_or_else(|e| log::error!("Could not save profile (for last used update): {e}"));
                log::info!("Updated last used timestamp of profile \"{}\"", profile.name);

                app.active_scene = SceneType::ViewProfile(SceneViewProfile {
                    mods: vec![],
                    profile: profile.clone(),
                    browser: Default::default(),
                    mod_details: Default::default(),
                });

                return Ok(iced::window::resize(app.main_window_id, WINDOW_SIZE_VIEW_PROFILE))
            },

            MsgHomePage::Login => {
                let temp_login_token: String = generate_token();
                let url: String = format!("{ACORN_BASE_URL}/goto_discord_auth?temp_login_token={}", temp_login_token);
                let ctx /* do NOT use explicit type here */ = copypasta::ClipboardContext::new().ok();

                app.active_scene = SceneType::Login(SceneLogin {
                    temp_login_token,
                    url,
                    clipboard_context: ctx,
                });
            },
        }
        Ok(Command::none())
    }

    fn view<'a>(&'a self, app: &'a MyApp) -> Result<Element<'a, Msg>, String> {
        let mut profiles: Vec<Element<Msg>> = Vec::new();
        for profile in app.profiles.iter() {
            profiles.push(profile.view(*COLOR_TEXT1, *COLOR_TEXT2));
            profiles.push(create_divider())
        }
        let profiles: Container<Msg> = container(column(profiles).spacing(5)).style(list_style);

        let main_content = container(
            iced::widget::column![
                    column![
                        row![
                            text("AcornGM").size(28).style(*COLOR_TEXT1),
                            Space::with_width(Length::Fill),
                            text(self.update_status_text).style(*COLOR_TEXT2),
                        ],
                        Space::with_height(4.0),
                        text("Recent Profiles").size(14).style(*COLOR_TEXT2).horizontal_alignment(alignment::Horizontal::Center),
                        Space::with_height(6.0),
                        scrollable(profiles).height(Length::Fill),
                    ]
                    .padding(20)
                ]
        ).align_x(alignment::Horizontal::Left);

        let button_bar = generate_button_bar(vec![
            button("Login").on_press(Msg::HomePage(MsgHomePage::Login)).into(),
        ], vec![
            button("Create Profile").on_press(Msg::HomePage(MsgHomePage::CreateProfile)).into(),
        ]);

        Ok(container(
            iced::widget::column![
                column![
                    main_content,
                ],
                button_bar
            ]
        ).into())
    }
}
