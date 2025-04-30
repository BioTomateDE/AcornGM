mod scenes;
mod utility;
mod default_file_paths;
mod ui_templates;
mod settings;

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use iced::{time, Application, Color, Command, Element, Font, Pixels, Size, Subscription};
use iced::Settings;
use log::{error, warn};
use biologischer_log::{init_logger, CustomLogger};
use iced::window::Icon;
use once_cell::sync::Lazy;
use crate::default_file_paths::{check_if_first_launch, get_home_directory, get_resource_image_path};
use crate::scenes::login::{MsgLogin, SceneLogin};
use crate::scenes::view_profile::{MsgViewProfile, SceneViewProfile};
use crate::utility::{get_device_info, show_error_dialogue, DeviceInfo};
use crate::scenes::homepage::{load_profiles, MsgHomePage, Profile, SceneHomePage};
use crate::scenes::create_profile::{MsgCreateProfile1, MsgCreateProfile2, SceneCreateProfile};
use crate::settings::{load_settings, AcornSettings};

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
    home_dir: PathBuf,
    app_root: PathBuf,
    device_info: DeviceInfo,
    settings: AcornSettings,
    profiles: Vec<Profile>,
    active_scene: SceneType,
    main_window_id: iced::window::Id,
    _logger: Arc<CustomLogger>,
}

#[derive(Clone)]
struct MyAppFlags {
    main_window_id: iced::window::Id,
    logger: Arc<CustomLogger>,
    app_root: PathBuf,
}

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
        let home_dir: PathBuf = get_home_directory(flags.logger.clone());
        let is_first_launch: bool = check_if_first_launch(&home_dir);
        let profiles: Vec<Profile> = load_profiles(&home_dir, is_first_launch).unwrap_or_else(|e| {
            show_error_dialogue("Could not get AcornGM profiles", &e);
            vec![]
        });
        let settings: AcornSettings = load_settings(&home_dir, is_first_launch).unwrap_or_else(|e| {
            show_error_dialogue(
                "Could not load AcornGM settings",
                &format!("Error while trying to load AcornGM settings: {e}\n\nThe program will use default settings."));
            Default::default()
        });
        let device_info: DeviceInfo = get_device_info();

        (Self {
            home_dir,
            app_root: flags.app_root,
            device_info,
            profiles,
            settings,
            active_scene: SceneType::HomePage(SceneHomePage {}),
            main_window_id: flags.main_window_id,
            _logger: flags.logger,
        }, Command::none())
    }
    fn title(&self) -> String {
        "AcornGM".to_string()
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        if let Msg::Global(msg) = message {
            return self.handle_global_messages(msg);
        }

        let scene_ptr = &mut self.active_scene as *mut SceneType;
        let app = self;
        
        // safe because scene is not read or written to while pattern matching; only by scene.update()
        unsafe {
            match &mut *scene_ptr {
                SceneType::HomePage(scene) => scene.update(app, message),
                SceneType::CreateProfile(scene) => scene.update(app, message),
                SceneType::ViewProfile(scene) => scene.update(app, message),
                SceneType::Login(scene) => scene.update(app, message),
            }
        }
    }
    fn view(&self) -> Element<Self::Message> {
        match &self.active_scene {
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
        if self.settings.access_token.is_none() && matches!(self.active_scene, SceneType::Login(_)) {
            return time::every(Duration::new(3, 0))
                .map(|_| Msg::Login(MsgLogin::SubRequestAccessToken))
        }
        time::every(Duration::new(10, 0))
            .map(|_| Msg::Global(MsgGlobal::Keepalive))
    }
}

impl MyApp {
    fn handle_global_messages(&mut self, message: MsgGlobal) -> Command<Msg> {
        match message {
            MsgGlobal::Keepalive => {}  // TODO send request
        }
        
        Command::none()
    }
}


pub fn main() -> iced::Result {
    let logger = init_logger(env!("CARGO_PKG_NAME"));

    let app_root: PathBuf = match std::env::current_exe() {
        Ok(exe_path) => exe_path
            .parent()
            .expect("Could not get parent directory of self executable file")
            .to_path_buf(),

        Err(e) => {
            error!("Could not get path of self executable file: {e}");
            logger.shutdown();
            std::process::exit(1);
        }
    };
    
    let icon_path: PathBuf = get_resource_image_path(&app_root, "logo.png");
    let icon: Option<Icon> = match iced::window::icon::from_file(icon_path) {
        Ok(icon) => Some(icon),
        Err(e) => {
            warn!("Could not load icon logo: {e}");
            None
        }
    };

    let window_settings = iced::window::Settings {
        size: WINDOW_SIZE_NORMAL,
        position: iced::window::Position::Centered,
        min_size: Some(Size{ width: 300.0, height: 300.0 }),
        max_size: None,
        visible: true,
        resizable: true,
        decorations: true,
        transparent: false,
        level: iced::window::Level::Normal,
        icon,
        platform_specific: iced::window::settings::PlatformSpecific::default(),
        exit_on_close_request: true,
    };

    let settings = Settings {
        id: Some("main".to_string()),
        window: window_settings,
        flags: MyAppFlags {
            main_window_id: iced::window::Id::unique(),
            logger,
            app_root,
        },
        fonts: vec![],
        default_font: Font::DEFAULT,
        default_text_size: Pixels(14.0),
        antialiasing: true,
    };

    MyApp::run(settings)
}

