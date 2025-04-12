use std::collections::HashMap;
use std::time::Instant;
use iced::{alignment, Command, Element};
use iced::widget::{container, row, column, text, button};
use crate::{Msg, MyApp, SceneType};
use webbrowser;
use crate::default_file_paths::show_msgbox;
use cli_clipboard::ClipboardProvider;
use serde::Deserialize;
use crate::scenes::homepage::SceneHomePage;
use crate::utility::{DeviceInfo, ACORN_BASE_URL};
use reqwest::blocking::Client as ReqClient;

#[derive(Debug, Clone)]
pub enum MsgLogin {
    BackToHomepage,
    Next,
    LoginExternal,
    CopyLink,
    SubRequestAccessToken(Instant),
}

#[derive(Default, Debug, Clone)]
pub struct SceneLogin {
    pub temp_login_token: String,
    pub url: String,
    pub request_listener_active: bool,
}


impl MyApp {
    pub fn update_login(&mut self, message: Msg) -> Command<Msg> {
        let scene: &mut SceneLogin = match &mut self.active_scene {
            SceneType::Login(scene) => scene,
            _ => {
                println!("[ERROR @ login::update]  Could not extract scene: {:?}", self.active_scene);
                return Command::none();
            }
        };

        match message {
            Msg::Login(MsgLogin::LoginExternal) => {
                if webbrowser::open(&scene.url).is_err() {
                    show_msgbox("Error while logging in", "Could not open URL to log in.");
                    return Command::none();
                }

                let access_token_arc = self.access_token.read().expect("Could not get access token RwLock reader");
                if access_token_arc.as_ref().is_none() {
                    scene.request_listener_active;
                }
            },

            Msg::Login(MsgLogin::BackToHomepage) => {
                self.active_scene = SceneType::HomePage(SceneHomePage {});
            },

            Msg::Login(MsgLogin::Next) => {
                self.active_scene = SceneType::HomePage(SceneHomePage {});
            },

            Msg::Login(MsgLogin::CopyLink) => {
                match cli_clipboard::ClipboardContext::new() {
                    Ok(mut ctx) => {
                        if let Err(error) = ctx.set_contents(scene.url.clone()) {
                            println!("[ERROR @ login::update]  Could not set contents of clipboard: {error}");
                        } else {
                            println!("[INFO @ login::update]  Set contents of clipboard to {}", scene.url);
                        }
                    },
                    Err(error) => {
                        println!("[ERROR @ login::update]  Could not initialize clipboard: {error}");
                        return Command::none();
                    },
                };
            },

            Msg::Login(MsgLogin::SubRequestAccessToken(_)) => {
                #[derive(Debug, Clone, Deserialize)]
                struct MyResponseJson {
                    access_token: String,
                }


                tokio::task::spawn(async move {
                    let temp_login_token: String = scene.temp_login_token.clone();
                    let device_info: DeviceInfo = self.device_info.clone();
                    let access_token = self.access_token.read().expect("Could not get RwLock reader");
                    let mut access_token_arc: Option<&String> = access_token.as_ref();

                    let mut body = HashMap::new();
                    body.insert("temp_login_token", temp_login_token);
                    body.insert("device_info", serde_json::to_string(&device_info).expect("Could not convert device info to json"));

                    let client = reqwest::Client::new();
                    let response = client
                        .post(format!("{ACORN_BASE_URL}/api/access_token"))
                        .json(&body)
                        .send()
                        .await;

                    if access_token_arc.is_some() { return }

                    if let Err(e) = response {
                        println!("[ERROR @ <async task>login::update]  Could not get access token from AcornGM server: {e}");
                        return
                    }
                    let resp: reqwest::Result<MyResponseJson> = response.unwrap().json().await;
                    if let Err(e) = resp {
                        println!("[ERROR @ <async task>login::update]  Could not get json from response: {e}");
                        return
                    }
                    // TODO check if this arc writing actually works
                    let access_token: String = resp.unwrap().access_token;
                    println!("[INFO @ <async task>login::update]  Got access token: {access_token}");
                    access_token_arc = Some(&access_token);
                });
            }
            _ => {},
        }
        Command::none()
    }

    pub fn view_login(&self, scene: &SceneLogin) -> Element<Msg> {
        let mut status_string: &'static str = "Idle";
        let access_token_arc = self.access_token.read().expect("Could not get RwLock reader");
        if access_token_arc.as_ref().is_some() {
            status_string = "Success";
        }
        if scene.request_listener_active {
            status_string = "In Browser";
        }

        let main_content = container(
            column![
                column![
                    text("Login to AcornGM").size(22).style(self.color_text1),
                    text("").size(4),
                    text(
                        "This will open your browser so you can log in.\n\
                        After you're done, return to this window and click the 'Next' button."
                    ).size(14).style(self.color_text1),
                    text("").size(4),
                    row![
                        button("Open Browser").on_press(Msg::Login(MsgLogin::LoginExternal)),
                        text("   ").size(10),
                        button("Copy Link").on_press(Msg::Login(MsgLogin::CopyLink)),
                    ],
                    text("").size(2),
                    column![
                        text("Alternatively, type out this link:").style(self.color_text2),
                        text("").size(2),
                        text(&scene.url).style(self.color_text1),
                    ],
                    text("").size(4),
                    row![
                        text("Status:").style(self.color_text2),
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


