use std::collections::HashMap;
use std::sync::Arc;
use iced::{alignment, Command, Element};
use iced::widget::{container, row, column, text, button};
use crate::{Msg, MyApp, Scene, SceneType, COLOR_TEXT1, COLOR_TEXT2};
use webbrowser;
use cli_clipboard::ClipboardProvider;
use log::error;
use serde::Deserialize;
use crate::scenes::homepage::SceneHomePage;
use crate::utility::{show_error_dialogue, ACORN_BASE_URL};
use reqwest::blocking::Client as ReqClient;
use crate::scenes::login::SceneLogin;

#[derive(Debug, Clone)]
pub enum MsgLogin {
    BackToHomepage,
    Next,
    LoginExternal,
    CopyLink,
    SubRequestAccessToken,
}


impl Scene for SceneLogin {
    fn update(&mut self, app: &mut MyApp, message: Msg) -> Command<Msg> {
        let message: MsgLogin = match message {
            Msg::Login(msg) => msg,
            other => {
                error!("Invalid message type {other:?}");
                return Command::none()
            }
        };

        match message {
            MsgLogin::LoginExternal => {
                self.do_external_login(app)
                    .unwrap_or_else(|e| show_error_dialogue("Error while logging in", &e))
            },

            MsgLogin::BackToHomepage => {
                app.active_scene = Arc::new(SceneType::HomePage(SceneHomePage {}));
            },

            MsgLogin::Next => {
                app.active_scene = Arc::new(SceneType::HomePage(SceneHomePage {}));
            },

            MsgLogin::CopyLink => {
                match cli_clipboard::ClipboardContext::new() {
                    Ok(mut ctx) => {
                        if let Err(error) = ctx.set_contents(self.url.clone()) {
                            println!("[ERROR @ login::update]  Could not set contents of clipboard: {error}");
                        } else {
                            println!("[INFO @ login::update]  Set contents of clipboard to {}", self.url);
                        }
                    },
                    Err(error) => {
                        println!("[ERROR @ login::update]  Could not initialize clipboard: {error}");
                        return Command::none();
                    },
                };
            },

            MsgLogin::SubRequestAccessToken => {
                #[derive(Debug, Clone, Deserialize)]
                struct MyResponseJson {
                    access_token: String,
                }

                let mut body = HashMap::new();
                body.insert("temp_login_token", self.temp_login_token.clone());
                body.insert("device_info", serde_json::to_string(&app.device_info).expect("Could not convert device info to json"));

                let client = ReqClient::new();
                let response = client
                    .post(format!("{ACORN_BASE_URL}/api/access_token"))
                    .json(&body)
                    .send();

                if app.access_token.is_some() { return Command::none() }

                if let Err(e) = response {
                    println!("[ERROR @ <async task>login::update]  Could not get access token from AcornGM server: {e}");
                    return Command::none()
                }
                let resp: reqwest::Result<MyResponseJson> = response.unwrap().json();
                if let Err(e) = resp {
                    println!("[ERROR @ <async task>login::update]  Could not get json from response: {e}");
                    return Command::none()
                }

                // request success; access token aquired
                let access_token = resp.unwrap().access_token;
                println!("[INFO @ <async task>login::update]  Got access token: {access_token}");
                app.access_token = Some(access_token);
            }
        }
        Command::none()
    }

    fn view<'a>(&self, app: &'a MyApp) -> Element<'a, Msg> {
        let mut status_string: &'static str = "Idle";
        if app.access_token.is_some() {
            status_string = "Success";
        }
        if self.request_listener_active {
            status_string = "In Browser";
        }

        let main_content = container(
            column![
                column![
                    text("Login to AcornGM").size(22).style(*COLOR_TEXT1),
                    text("").size(4),
                    text(
                        "This will open your browser so you can log in.\n\
                        After you're done, return to this window and click the 'Next' button."
                    ).size(14).style(*COLOR_TEXT1),
                    text("").size(4),
                    row![
                        button("Open Browser").on_press(Msg::Login(MsgLogin::LoginExternal)),
                        text("   ").size(10),
                        button("Copy Link").on_press(Msg::Login(MsgLogin::CopyLink)),
                    ],
                    text("").size(2),
                    column![
                        text("Alternatively, type out this link:").style(*COLOR_TEXT2),
                        text("").size(2),
                        text(&self.url).style(*COLOR_TEXT1),
                    ],
                    text("").size(4),
                    row![
                        text("Status:").style(*COLOR_TEXT2),
                        text("   ").size(10),
                        text(status_string),
                    ]
                    ].spacing(10),
                ]
                .padding(20)
        ).align_x(alignment::Horizontal::Left);

        let button_bar = container(
            row![
                container(
                    row![
                        text("    ").size(10),
                        button("Cancel").on_press(Msg::Login(MsgLogin::BackToHomepage)),
                    ]
                    .spacing(10)
                ),
                text("                                                                  ").size(20),
                container(
                     row![
                        button("Next >").on_press(Msg::Login(MsgLogin::Next)),
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


impl SceneLogin {
    fn do_external_login(&mut self, app: &MyApp) -> Result<(), String> {
        if app.access_token.is_some() {
            return Err("Already logged in!".to_string())
        }

        webbrowser::open(&self.url)
            .map_err(|e| format!("Failed to open browser: {e}"))?;
            
        self.request_listener_active = true;
        Ok(())
    }
}

