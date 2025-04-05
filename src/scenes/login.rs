use iced::{alignment, Element};
use iced::widget::{container, row, column, text, button};
use crate::{Msg, MyApp, SceneType};
use webbrowser;
use crate::default_file_paths::show_msgbox;
use crate::utility::BASE_URL;
use getrandom;
use base64::prelude::*;
use iced::futures::SinkExt;
use iced::futures::task::SpawnExt;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use crate::scenes::homepage::SceneHomePage;

#[derive(Debug, Clone)]
pub enum MsgLogin {
    BackToHomepage,
    LoginExternal,
}

#[derive(Default, Debug)]
pub struct SceneLogin {
    pub temp_login_token: Option<String>,
    pub auth_code: Option<String>,
    pub status_string: &'static str,
    // pub receiver: Option<UnboundedReceiver<Msg>>,
}


impl MyApp {
    pub fn update_login(&mut self, message: Msg) {
        let scene: &mut SceneLogin = match &mut self.active_scene {
            SceneType::Login(scene) => scene,
            _ => {
                println!("[ERROR @ login::update]  Could not extract scene: {:?}", self.active_scene);
                return;
            }
        };

        match message {
            Msg::Login(MsgLogin::LoginExternal) => {
                let mut buf = [0u8; 64];
                if getrandom::fill(&mut buf).is_err() {
                    show_msgbox("Error while logging in", "Could not generate temporary login token.");
                    return;
                };
                let temp_login_token: String = BASE64_STANDARD.encode(buf);

                if webbrowser::open( &format!("{}login.html?tempLoginToken={}", BASE_URL, temp_login_token)).is_err() {
                    show_msgbox("Error while logging in", "Could not open URL to log in.");
                    return;
                }
                scene.temp_login_token = Some(temp_login_token.clone());
                scene.status_string = "Browser opened";

                println!("t");
                // let (mut sender, receiver): (UnboundedSender<Msg>, UnboundedReceiver<Msg>) = unbounded_channel();
                // scene.receiver = Some(receiver);
                let sender = self.sender.clone();
                std::thread::spawn(move || async move {
                    println!("thread spawned");
                    let temp_login_token = temp_login_token.clone();
                    loop {
                        std::thread::sleep(core::time::Duration::from_millis(420));
                        println!("loop iteration");
                        let req = reqwest::get(format!("{BASE_URL}check_callback?tempLoginToken={temp_login_token}")).await;
                        let res = match req {
                            Err(error) => { println!("Error while requesting auth code: {error}"); continue },
                            Ok(res) => res
                        };
                        let text = match res.text().await {
                            Err(error) => { println!("Error while getting text from auth code request: {error}"); continue },
                            Ok(text) => text,
                        };
                        if text != "" && text != "no" {
                            let _ = match sender.send(Msg::Login(MsgLogin::LoginExternal)) {
                                Err(error) => { println!("Error while sending Message for auth code: {error}"); continue },
                                Ok(text) => text,
                            };
                            // scene.auth_code = Some(text);
                            return
                        }
                    }
                });
                println!("s");
            },

            Msg::Login(MsgLogin::BackToHomepage) => {
                self.active_scene = SceneType::HomePage(SceneHomePage {});
            },

            _ => {},
        }
    }

    pub fn view_login(&self, scene: &SceneLogin) -> Element<Msg> {
        let main_content = container(
            column![
                column![
                    text("Login to AcornGM").size(22).style(self.color_text1),
                    text("").size(10),
                    text("This will open your browser so you can log in.\nAfter you're done, return to this window.").size(14).style(self.color_text1),
                    button("Open Browser").on_press(Msg::Login(MsgLogin::LoginExternal)),
                    text("").size(10),
                    text(&scene.status_string),
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
                        button("Next >").on_press(Msg::Login(MsgLogin::BackToHomepage)),
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

