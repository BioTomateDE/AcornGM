mod scenes;
mod utility;
mod default_file_paths;

use iced::{Application, Color, Command, Font, Pixels, Sandbox, Size};
use crate::scenes::create_profile1::{MsgCreateProfile1, SceneCreateProfile};
use crate::scenes::homepage::{load_profiles, MsgHomePage, Profile, SceneHomePage};
use iced::Settings;
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


#[derive(Debug, Clone)]
enum SceneType {
    HomePage(SceneHomePage),
    CreateProfile1(SceneCreateProfile),
    CreateProfile2(SceneCreateProfile),
    ViewProfile(SceneViewProfile),
    Login(SceneLogin),
}


#[derive(Debug, Clone)]
struct MyApp {
    flags: MyAppFlags,
    profiles: Vec<Profile>,
    active_scene: SceneType,
    color_text1: Color,
    color_text2: Color,
    color_text_red: Color,
}

#[derive(Debug, Clone)]
struct MyAppFlags {
    main_window_id: iced::window::Id,
}

impl Application for MyApp {
    type Executor = iced::executor::Default;
    type Message = Msg;
    type Theme = iced::Theme;
    type Flags = MyAppFlags;

    fn new(flags: Self::Flags) -> (MyApp, Command<Msg>) {
        let profiles: Vec<Profile> = load_profiles();
        let ts: MyApp = Self {
            flags,
            profiles,
            active_scene: SceneType::HomePage(SceneHomePage {}),
            color_text1: Color::from_rgb8(231, 227, 213),
            color_text2: Color::from_rgb8(147, 146, 145),
            color_text_red: Color::from_rgb8(237, 49, 31),
        };
        let pmo: Command<Msg> = Command::none();
        (ts, pmo)
    }
    fn title(&self) -> String {
        "AcornGM".to_string()
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match &self.active_scene {
            SceneType::HomePage(_) => return self.update_homepage(message),
            SceneType::CreateProfile1(_) => return self.update_create_profile1(message),
            SceneType::CreateProfile2(_) => return self.update_create_profile2(message),
            SceneType::ViewProfile(_) => return self.update_view_profile(message),
            SceneType::Login(_) => return self.update_login(message),
        }
        Command::none()
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
}


pub fn main() -> iced::Result {
    let main_window_id: iced::window::Id = iced::window::Id::unique();

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

    let settings = Settings {
        id: Some("main".to_string()),
        window: window_settings,
        flags: MyAppFlags {main_window_id},
        fonts: vec![],
        default_font: Font::DEFAULT,
        default_text_size: Pixels(14.0),
        antialiasing: true,
    };

    MyApp::run(settings)
}

