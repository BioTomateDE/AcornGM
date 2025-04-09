use std::path::PathBuf;
use iced::{alignment, Element};
use iced::widget::{container, row, column, text, button};
use crate::{Msg, MyApp, SceneType};
use webbrowser;
use crate::default_file_paths::{get_home_directory, show_msgbox};
use crate::utility::BASE_URL;
use getrandom;
use base64::prelude::*;
use crate::scenes::homepage::SceneHomePage;

#[derive(Debug, Clone)]
pub enum MsgLogin {
    BackToHomepage,
    LoginExternal,
    Next,
}

#[derive(Default, Debug, Clone)]
pub struct SceneLogin {
    pub temp_login_token: Option<String>,
    pub status_string: &'static str,
}


impl MyApp {
    pub fn update_login(&mut self, message: Msg) -> iced::Command<Msg> {
        let scene: &mut SceneLogin = match &mut self.active_scene {
            SceneType::Login(scene) => scene,
            _ => {
                println!("[ERROR @ login::update]  Could not extract scene: {:?}", self.active_scene);
                return iced::Command::none();
            }
        };

        match message {
            Msg::Login(MsgLogin::LoginExternal) => {
                let mut buf = [0u8; 64];
                if getrandom::fill(&mut buf).is_err() {
                    show_msgbox("Error while logging in", "Could not generate temporary login token.");
                    return iced::Command::none();
                };
                let temp_login_token: String = BASE64_URL_SAFE.encode(buf);

                if webbrowser::open( &format!("{}login.html?tempLoginToken={}", BASE_URL, temp_login_token)).is_err() {
                    show_msgbox("Error while logging in", "Could not open URL to log in.");
                    return iced::Command::none();
                }
                scene.temp_login_token = Some(temp_login_token.clone());

                if scene.status_string != "Browser opened" {
                    std::thread::spawn(move || check_callback(&temp_login_token));
                };
                scene.status_string = "Browser opened";
                iced::Command::none()
            },

            Msg::Login(MsgLogin::BackToHomepage) => {
                self.active_scene = SceneType::HomePage(SceneHomePage {});
                iced::Command::none()
            },

            Msg::Login(MsgLogin::Next) => {
                let access_token_file_path: PathBuf = get_home_directory().join("./access_token.txt");
                if !access_token_file_path.is_file() {
                    // server didn't respond yet
                    show_msgbox("Login is in process", "Login process is not done yet! Please complete it in your browser.");
                    return iced::Command::none();
                }

                let access_token: String = match std::fs::read_to_string(access_token_file_path) {
                    Ok(text) => text,
                    Err(error) => {
                        show_msgbox("Error reading file", &format!("Could not read access token file: {error}"));
                        return iced::Command::none();
                    },
                };
                let access_token: &str = access_token.trim();
                println!("Access Token: {access_token}");
                scene.status_string = "Success";

                iced::window::resize(iced::window::Id::unique(), iced::Size {width: 900.0, height: 500.0})
                // self.active_scene = SceneType::HomePage(SceneHomePage {});
            },

            _ => iced::Command::none(),
        }
    }

    pub fn view_login(&self, scene: &SceneLogin) -> Element<Msg> {
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
                    button("Open Browser").on_press(Msg::Login(MsgLogin::LoginExternal)),
                    text("").size(10),
                    row![
                        text("Status:").style(self.color_text2),
                        text("    ").size(10),
                        text(&scene.status_string),
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



// warning: terrible code
fn check_callback(temp_login_token: &str) -> Result<(), reqwest::Error> {
    // check if already running
    match get_thread_info() {
        Err(error) => {
            println!("[ERROR @ <thread>login::check_callback]  Error getting thread info: {error}");
            return Ok(());
        }
        Ok(thread_info) => {
            if thread_info == "running" {
                println!("[INFO @ <thread>login::check_callback]  Didn't start this thread as it's already running.");
                return Ok(());
            }
        }
    };

    // set to running
    if let Err(error) = write_thread_info("running") {
        println!("[ERROR @ <thread>login::check_callback]  Error writing to thread info file: {error}");
    };

    // get callback in interval
    loop {
        // Check if cancelled
        match get_thread_info() {
            Err(error) => {
                println!("[ERROR @ <thread>login::check_callback]  Error getting thread info: {error}");
                return Ok(());
            }
            Ok(thread_info) => {
                if thread_info == "cancel" {
                    println!("[INFO @ <thread>login::check_callback]  Cancelled.");
                    if let Err(error) = delete_thread_info() {
                        println!("[ERROR @ <thread>login::check_callback]  Error deleting thread info file: {error}");
                    };
                    return Ok(());
                }
            }
        };

        std::thread::sleep(core::time::Duration::from_millis(420));
        let text = reqwest::blocking::get(format!("{BASE_URL}check_callback?tempLoginToken={temp_login_token}"))?.text()?;
        // println!("{temp_login_token} | {text}");

        if text != "" && text != "no" {
            // write to file
            let access_token_file_path: PathBuf = get_home_directory().join("./access_token.txt");
            if let Err(error) = std::fs::write(access_token_file_path, text) {
                println!("[ERROR @ <thread>login::check_callback]  Error writing to access token code file: {error}");
            };
            return Ok(())
        }
    }
}


fn get_thread_info() -> Result<String, String> {
    let thread_info_file_path: PathBuf = get_home_directory().join("./temp_thread_login.txt");
    if !thread_info_file_path.exists() {
        return Ok("idle".to_string())
    }

    match std::fs::read_to_string(thread_info_file_path) {
        Ok(thread_info) => Ok(thread_info),
        Err(error) => Err(format!("Could not read callback thread info file: {error}"))
    }
}

fn write_thread_info(status: &str) -> Result<(), String> {
    let thread_info_file_path: PathBuf = get_home_directory().join("./temp_thread_login.txt");
    if !thread_info_file_path.exists() {
        return Ok(())
    }

    match std::fs::write(thread_info_file_path, status) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("Could not write to callback thread info file: {error}"))
    }
}

fn delete_thread_info() -> Result<(), String> {
    let thread_info_file_path: PathBuf = get_home_directory().join("./temp_thread_login.txt");
    if !thread_info_file_path.exists() {
        return Ok(())
    }

    match std::fs::remove_file(thread_info_file_path) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("Could not delete callback thread info file: {error}"))
    }
}
