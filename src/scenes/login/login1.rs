use copypasta::ClipboardProvider;
use iced::{alignment, Command, Element};
use iced::widget::{container, row, column, text, button, Space};
use crate::{Msg, MyApp, Scene, SceneType, COLOR_TEXT1, COLOR_TEXT2};
use webbrowser;
use log::{error, info, warn};
use reqwest::StatusCode;
use serde::Deserialize;
use crate::panic_catcher::catch_panic;
use crate::scenes::homepage::SceneHomePage;
use crate::utility::{show_error_dialogue, ACORN_API_URL};
use crate::scenes::login::SceneLogin;
use crate::settings::save_settings;
use crate::ui_templates::generate_button_bar;

#[derive(Debug, Clone)]
pub enum MsgLogin {
    BackToHomepage,
    Next,
    LoginExternal,
    CopyLink,
    SubRequestAccessToken,
    AsyncResponseAccessToken(Option<String>),
}


impl Scene for SceneLogin {
    fn update(&mut self, app: &mut MyApp, message: Msg) -> Result<Command<Msg>, String> {
        let message: MsgLogin = match message {
            Msg::Login(msg) => msg,
            other => return Err(format!("Invalid message type {other:?} for Login")),
        };

        match message {
            MsgLogin::LoginExternal => {
                self.do_external_login(app)
                    .unwrap_or_else(|e| show_error_dialogue("Error while logging in", &e))
            }
            MsgLogin::BackToHomepage => {
                app.active_scene = SceneType::HomePage(SceneHomePage {
                    update_status_text: ""
                });
            }
            MsgLogin::Next => {
                // TODO
                app.active_scene = SceneType::HomePage(SceneHomePage {
                    update_status_text: "",
                });
            }
            MsgLogin::CopyLink => {
                let Some(ref mut ctx) = self.clipboard_context else {
                    return Err("Clipboard seems to be not supported in your environment".to_string())
                };
                ctx.set_contents(self.url.clone())
                    .map_err(|e| format!("Could not set clipboard contents: {e}"))?;
                info!("Set clipboard contents to {}", self.url);
            }
            MsgLogin::SubRequestAccessToken => {
                return Ok(self.sub_request_access_token(app))
            }
            MsgLogin::AsyncResponseAccessToken(Some(token)) => {
                info!("Got access token: {token}");
                app.settings.access_token = Some(token);
                save_settings(&app.home_dir, &app.settings).unwrap_or_else(|e|
                    show_error_dialogue("Could not save AcornGM settings", &format!("Failed to save AcornGM settings: {e}")))
            }
            MsgLogin::AsyncResponseAccessToken(None) => {}
        }
        Ok(Command::none())
    }

    fn view<'a>(&self, app: &'a MyApp) -> Result<Element<'a, Msg>, String> {
        let status_string: &'static str = if app.settings.access_token.is_some() {"Logged in"} else {"Not logged in"};

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
                        Space::with_width(6.0),
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
                        Space::with_width(6.0),
                        text(status_string),
                    ]
                    ].spacing(10),
                ]
                .padding(20)
        ).align_x(alignment::Horizontal::Left);

        let button_bar = generate_button_bar(vec![
            button("Cancel").on_press(Msg::Login(MsgLogin::BackToHomepage)).into(),
        ], vec![
            button("Next >").on_press(Msg::Login(MsgLogin::Next)).into(),
        ]);

        Ok(container(
            column![
                column![
                    main_content,
                ],
                button_bar
            ]
        ).into())
    }
}


impl SceneLogin {
    fn do_external_login(&mut self, app: &MyApp) -> Result<(), String> {
        if app.settings.access_token.is_some() {
            return Err("Already logged in!".to_string())
        }

        webbrowser::open(&self.url)
            .map_err(|e| format!("Failed to open browser: {e}"))?;

        Ok(())
    }
    
    fn sub_request_access_token(&mut self, app: &mut MyApp) -> Command<Msg> {
        if app.settings.access_token.is_some() {
            return Command::none()
        }
        
        catch_panic(|| Command::perform(request_access_token(self.temp_login_token.clone()),
            |result| Msg::Login(MsgLogin::AsyncResponseAccessToken(result)),
        ))
    }
}


async fn request_access_token(temp_login_token: String) -> Option<String> {
    #[derive(Debug, Deserialize)]
    struct SuccessResponseJson {
        access_token: String,
    }
    #[derive(Debug, Deserialize)]
    struct ErrorResponseJson {
        error: String,
    }

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{ACORN_API_URL}/access_token"))
        .json(&temp_login_token)
        .send()
        .await;

    let resp = match resp {
        Ok(resp) => resp,
        Err(e) => {
            error!("Request error: {e}");
            return None
        }
    };

    let status: StatusCode = resp.status();
    if status.is_client_error() {
        let body: String = resp.text().await.unwrap_or_else(|_| "<invalid string>".to_string());
        match serde_json::from_str::<ErrorResponseJson>(&body) {
            Ok(json) => {
                if status.as_u16() != 404 {
                    // if status is 404 and body is valid json error, it means that temp_login_token doesn't exist yet
                    // because the user is still in the login process (not finished yet)
                    error!("Client error response {}: {}", status.formatted(), json.error);
                }
            }
            Err(_) => error!("(Raw) Client error response {}: {}", status.formatted(), body),
        }
        return None
    }
    if status.is_server_error() {
        let body: String = resp.text().await.unwrap_or_else(|_| "<invalid string>".to_string());
        match serde_json::from_str::<ErrorResponseJson>(&body) {
            Ok(json) => warn!("Server error response {}: {}", status.formatted(), json.error),
            Err(_) => warn!("(Raw) Server error response {}: {}", status.formatted(), body),
        }
        return None
    }

    let json = match resp.json::<SuccessResponseJson>().await {
        Ok(json) => json,
        Err(e) => {
            error!("JSON parse error: {e}");
            return None
        }
    };
    info!("Response json: {json:?}");

    Some(json.access_token)
}

trait StatusCodeFmt {
    fn formatted(&self) -> String;
}
impl StatusCodeFmt for StatusCode {
    fn formatted(&self) -> String {
        format!("{} - {}", self.as_u16(), self.canonical_reason().unwrap_or("<unknown status>"))
    }
}


