mod scenes;
mod utility;
mod default_file_paths;

use std::path::PathBuf;
use iced::{Application, Color, Command, Font, Pixels, Size};
use crate::scenes::create_profile1::{MsgCreateProfile1, SceneCreateProfile};
use crate::scenes::homepage::{load_profiles, MsgHomePage, Profile, SceneHomePage};
use iced::Settings;
use crate::default_file_paths::get_home_directory;
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
    home_dir: PathBuf,
    current_working_dir: PathBuf,
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


const WINDOW_SIZE_NORMAL: Size = Size { width: 500.0, height: 500.0 };
const WINDOW_SIZE_VIEW_PROFILE: Size = Size { width: 900.0, height: 800.0 };

impl Application for MyApp {
    type Executor = iced::executor::Default;
    type Message = Msg;
    type Theme = iced::Theme;
    type Flags = MyAppFlags;

    fn new(flags: Self::Flags) -> (MyApp, Command<Msg>) {
        let home_dir: PathBuf = get_home_directory();
        let current_working_dir: PathBuf = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(error) => {
                println!("[FATAL @ main::MyApp::new]  Could not get current working directory: {error}");
                std::process::exit(1);
            },
        };
        let profiles: Vec<Profile> = load_profiles(&home_dir);

        let ts: MyApp = Self {
            flags,
            home_dir,
            current_working_dir,
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
            SceneType::HomePage(_) => self.update_homepage(message),
            SceneType::CreateProfile1(_) => self.update_create_profile1(message),
            SceneType::CreateProfile2(_) => self.update_create_profile2(message),
            SceneType::ViewProfile(_) => self.update_view_profile(message),
            SceneType::Login(_) => self.update_login(message),
        }
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
        size: WINDOW_SIZE_NORMAL,
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

