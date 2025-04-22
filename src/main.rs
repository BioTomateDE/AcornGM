mod scenes;
mod utility;
mod default_file_paths;

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use iced::{time, Application, Color, Command, Element, Font, Pixels, Size, Subscription};
use iced::Settings;
use log::error;
use biologischer_log::{init_logger, CustomLogger};
use once_cell::sync::Lazy;
use crate::default_file_paths::get_home_directory;
use crate::scenes::login::{MsgLogin, SceneLogin};
use crate::scenes::view_profile::{MsgViewProfile, SceneViewProfile};
use crate::utility::{get_device_info, DeviceInfo};
use crate::scenes::homepage::{load_profiles, MsgHomePage, Profile, SceneHomePage};
use crate::scenes::create_profile::{MsgCreateProfile1, MsgCreateProfile2, SceneCreateProfile};

#[derive(Debug, Clone)]
enum Msg {
    Global(MsgGlobal),
    HomePage(MsgHomePage),
    CreateProfile1(MsgCreateProfile1),
    CreateProfile2(MsgCreateProfile2),
    ViewProfile(MsgViewProfile),
    Login(MsgLogin),
}

#[derive(Debug, Clone)]
enum MsgGlobal {
    Keepalive,
}

trait Scene {
    fn update(&mut self, app: &mut MyApp, message: Msg) -> Command<Msg>;
    fn view<'a>(&'a self, app: &'a MyApp) -> Element<'a, Msg>;
}

#[derive(Debug, Clone)]
enum SceneType {
    HomePage(SceneHomePage),
    CreateProfile(SceneCreateProfile),
    ViewProfile(SceneViewProfile),
    Login(SceneLogin),
}


#[derive(Clone)]
struct MyApp {
    flags: MyAppFlags,
    home_dir: PathBuf,
    app_root: PathBuf,
    device_info: DeviceInfo,
    profiles: Vec<Profile>,
    access_token: Option<String>,
    active_scene: Arc<SceneType>,
}

#[derive(Clone)]
struct MyAppFlags {
    main_window_id: iced::window::Id,
    logger: Arc<CustomLogger>,
}

// const COLOR_TEXT1: Color = Color::from_rgb8(231, 227, 213);
// const COLOR_TEXT2: Color = Color::from_rgb8(147, 146, 145);
// const COLOR_TEXT_RED: Color = Color::from_rgb8(237, 49, 31);
const COLOR_TEXT1: Lazy<Color> = Lazy::new(|| Color::new(0.906, 0.890, 0.835, 1.0));
const COLOR_TEXT2: Lazy<Color> = Lazy::new(|| Color::new(0.576, 0.573, 0.569, 1.0));
const COLOR_TEXT_RED: Lazy<Color> = Lazy::new(|| Color::new(0.929, 0.192, 0.122, 1.0));

const WINDOW_SIZE_NORMAL: Size = Size { width: 500.0, height: 500.0 };
const WINDOW_SIZE_VIEW_PROFILE: Size = Size { width: 900.0, height: 800.0 };

impl Application for MyApp {
    type Executor = iced::executor::Default;
    type Message = Msg;
    type Theme = iced::Theme;
    type Flags = MyAppFlags;

    fn new(flags: Self::Flags) -> (MyApp, Command<Msg>) {
        let home_dir: PathBuf = get_home_directory();
        let app_root: PathBuf = match std::env::current_exe() {
            Ok(exe_path) => exe_path
                .parent()
                .expect("Could not get parent directory of self executable file")
                .to_path_buf(),

            Err(e) => {
                error!("Could not get path of self executable file: {e}");
                std::process::exit(1);
            }
        };
        let profiles: Vec<Profile> = load_profiles(&home_dir);
        let device_info: DeviceInfo = get_device_info();

        let ts: MyApp = Self {
            flags,
            home_dir,
            app_root,
            device_info,
            profiles,
            access_token: None,   // TODO load from file
            active_scene: Arc::new(SceneType::HomePage(SceneHomePage {})),
        };
        let pmo: Command<Msg> = Command::none();
        (ts, pmo)
    }
    fn title(&self) -> String {
        "AcornGM".to_string()
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match &*self.active_scene {
            SceneType::HomePage(scene) => scene.clone().update(self, message),
            SceneType::CreateProfile(scene) => scene.clone().update(self, message),
            SceneType::ViewProfile(scene) => scene.clone().update(self, message),
            SceneType::Login(scene) => scene.clone().update(self, message),
        }
    }
    fn view(&self) -> Element<Self::Message> {
        match &*self.active_scene {
            SceneType::HomePage(scene) => scene.view(self),
            SceneType::CreateProfile(scene) => scene.view(self),
            SceneType::ViewProfile(scene) => scene.view(self),
            SceneType::Login(scene) => scene.view(self),
        }
    }
    fn theme(&self) -> iced::Theme {
        iced::Theme::GruvboxDark
    }
    fn subscription(&self) -> Subscription<Msg> {
        if let SceneType::Login(scene) = &*self.active_scene {
            if scene.request_listener_active {
                return time::every(Duration::new(3, 0))
                    .map(|_| Msg::Login(MsgLogin::SubRequestAccessToken))
            }
        }
        time::every(Duration::new(10, 0))
            .map(|_| Msg::Global(MsgGlobal::Keepalive))
    }
}


pub fn main() -> iced::Result {
    let logger = init_logger();

    let main_window_id: iced::window::Id = iced::window::Id::unique();

    let window_settings = iced::window::Settings {
        size: WINDOW_SIZE_NORMAL,
        position: iced::window::Position::Centered,
        min_size: Some(Size{ width: 300.0, height: 300.0 }),
        max_size: None,
        visible: true,
        resizable: false,   // change later
        decorations: true,
        transparent: false,
        level: iced::window::Level::Normal,
        icon: None,     // TODO
        platform_specific: iced::window::settings::PlatformSpecific::default(),
        exit_on_close_request: true,
    };

    let settings = Settings {
        id: Some("main".to_string()),
        window: window_settings,
        flags: MyAppFlags {
            main_window_id,
            logger,
        },
        fonts: vec![],
        default_font: Font::DEFAULT,
        default_text_size: Pixels(14.0),
        antialiasing: true,
    };

    MyApp::run(settings)
}

