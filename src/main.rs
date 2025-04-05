mod scenes;
mod utility;
mod default_file_paths;

use iced::{Application, Color, Font, Pixels, Sandbox, Size};
use crate::scenes::create_profile1::{MsgCreateProfile1, SceneCreateProfile};
use crate::scenes::homepage::{load_profiles, MsgHomePage, Profile, SceneHomePage};
use iced::Settings;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use crate::scenes::create_profile2::MsgCreateProfile2;
use crate::scenes::login::{MsgLogin, SceneLogin};
use crate::scenes::view_profile::{MsgViewProfile, SceneViewProfile};

#[derive(Debug, Clone)]
enum Msg {
    _ProfilesLoaded(Vec<Profile>),
    HomePage(MsgHomePage),
    CreateProfile1(MsgCreateProfile1),
    CreateProfile2(MsgCreateProfile2),
    ViewProfile(MsgViewProfile),
    Login(MsgLogin),
}


#[derive(Debug)]
enum SceneType {
    HomePage(SceneHomePage),
    CreateProfile1(SceneCreateProfile),
    CreateProfile2(SceneCreateProfile),
    ViewProfile(SceneViewProfile),
    Login(SceneLogin),
}


#[derive(Debug)]
struct MyApp {
    profiles: Vec<Profile>,
    active_scene: SceneType,
    color_text1: Color,
    color_text2: Color,
    color_text_red: Color,
    // receiver: std::cell::RefCell<Option<UnboundedReceiver<Msg>>>,
    // sender: std::cell::RefCell<Option<UnboundedSender<Msg>>>,
    receiver: UnboundedReceiver<Msg>,
    sender: UnboundedSender<Msg>,
}

struct MyAppFlags {
    receiver: UnboundedReceiver<Msg>,
    sender: UnboundedSender<Msg>,
}

impl Application for MyApp {
    type Executor = iced::executor::Default;
    type Message = Msg;
    type Theme = iced::Theme;
    type Flags = MyAppFlags;
    fn new(flags: Self::Flags) -> (MyApp, iced::Command<Msg>) {
        let profiles: Vec<Profile> = load_profiles();
        let my_app = Self {
            profiles,
            active_scene: SceneType::HomePage(SceneHomePage {}),
            color_text1: Color::from_rgb8(231, 227, 213),
            color_text2: Color::from_rgb8(147, 146, 145),
            color_text_red: Color::from_rgb8(237, 49, 31),
            // receiver: std::cell::RefCell::new(Some(flags.receiver)),
            // sender: std::cell::RefCell::new(Some(flags.sender)),
            receiver: flags.receiver,
            sender: flags.sender,
        };
        (my_app, iced::Command::none())
    }
    fn title(&self) -> String {
        "AcornGM".to_string()
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<Msg> {
        match &self.active_scene {
            SceneType::HomePage(_) => self.update_homepage(message),
            SceneType::CreateProfile1(_) => self.update_create_profile1(message),
            SceneType::CreateProfile2(_) => self.update_create_profile2(message),
            SceneType::ViewProfile(_) => self.update_view_profile(message),
            SceneType::Login(_) => self.update_login(message),
        }
        iced::Command::none()
    }
    fn view(&self) -> iced::Element<Self::Message> {
        match &self.active_scene {
            SceneType::HomePage(_) => self.view_homepage(),
            SceneType::CreateProfile1(scene) => self.view_create_profile1(scene),
            SceneType::CreateProfile2(scene) => self.view_create_profile2(scene),
            SceneType::ViewProfile(scene) => self.view_view_profile(scene),
            SceneType::Login(scene) => self.view_login(scene),
        }
    }
    fn theme(&self) -> iced::Theme {
        iced::Theme::GruvboxDark
    }
    fn subscription(&self) -> iced::Subscription<Msg> {
        iced::subscription::unfold(
            "login thingy",
            &self.receiver,
            move |mut receiver| async move {
                let msg = receiver.recv().await.unwrap();
                (msg, receiver)
            },
        )
    }
}


pub fn main() -> iced::Result {
    let window_settings = iced::window::Settings {
        size: Size{ width: 500.0, height: 500.0 },
        position: iced::window::Position::Centered,
        min_size: Some(Size{ width: 300.0, height: 300.0 }),
        max_size: None,
        visible: true,
        resizable: false,
        decorations: true,
        transparent: false,
        level: iced::window::Level::Normal,
        icon: None,     // TODO
        platform_specific: iced::window::settings::PlatformSpecific {
            application_id: "AcornGM".to_string(),
        },
        exit_on_close_request: true,
    };

    let (sender, receiver): (UnboundedSender<Msg>, UnboundedReceiver<Msg>) = unbounded_channel::<Msg>();

    let settings = Settings::<MyAppFlags> {
        id: Some("ts id pmo".to_string()),
        window: window_settings,
        flags: MyAppFlags { sender, receiver },
        fonts: vec![],
        default_font: Font::DEFAULT,
        default_text_size: Pixels(14.0),
        antialiasing: true,
    };

    MyApp::run(settings)
}

