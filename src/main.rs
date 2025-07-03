mod scenes;
mod utility;
mod default_file_paths;
mod ui_templates;
mod settings;
mod resources;
mod updater;

use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use iced::{time, Application, Color, Command, Element, Font, Pixels, Size, Subscription};
use iced::Settings;
use iced::widget::text;
use log::{error, warn};
use iced::window::Icon;
use once_cell::sync::Lazy;
use crate::default_file_paths::{check_if_first_launch, get_home_directory};
use crate::resources::APP_ICON;
use crate::scenes::login::{MsgLogin, SceneLogin};
use crate::scenes::view_profile::{MsgViewProfile, SceneViewProfile};
use crate::utility::show_error_dialogue;
use crate::scenes::homepage::{load_profiles, MsgHomePage, Profile, SceneHomePage};
use crate::scenes::create_profile::{MsgCreateProfile1, MsgCreateProfile2, SceneCreateProfile};
use crate::settings::{load_settings, AcornSettings};
use crate::updater::{check_for_updates, download_update_file};

#[allow(unused_imports)]
use async_std as _;     // makes iced commands not panic (enables tokio 1.0 runtime or smth)

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
    CheckedForUpdate(Result<Option<String>, String>),
    DownloadedUpdateFile(Result<(), String>),
}

trait Scene {
    fn update(&mut self, app: &mut MyApp, message: Msg) -> Result<Command<Msg>, String>;
    fn view<'a>(&'a self, app: &'a MyApp) -> Result<Element<'a, Msg>, String>;
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
    settings: AcornSettings,
    profiles: Vec<Profile>,
    active_scene: SceneType,
    main_window_id: iced::window::Id,
}

#[derive(Clone)]
struct MyAppFlags {
    main_window_id: iced::window::Id,
    app_root: PathBuf,
}

static COLOR_TEXT1: Lazy<Color> = Lazy::new(|| Color::new(0.906, 0.890, 0.835, 1.0));
static COLOR_TEXT2: Lazy<Color> = Lazy::new(|| Color::new(0.576, 0.573, 0.569, 1.0));
static COLOR_TEXT_RED: Lazy<Color> = Lazy::new(|| Color::new(0.929, 0.192, 0.122, 1.0));

const WINDOW_SIZE_NORMAL: Size = Size { width: 500.0, height: 500.0 };
const WINDOW_SIZE_VIEW_PROFILE: Size = Size { width: 900.0, height: 800.0 };

impl Application for MyApp {
    type Executor = iced::executor::Default;
    type Message = Msg;
    type Theme = iced::Theme;
    type Flags = MyAppFlags;

    fn new(flags: Self::Flags) -> (MyApp, Command<Msg>) {
        let home_dir: PathBuf = get_home_directory();
        let is_first_launch: bool = check_if_first_launch(&home_dir);

        if let Err(e) = fs::create_dir_all(&home_dir) {
            show_error_dialogue("Could not create AcornGM home directory", &format!("Error while trying to create AcornGM home directory: {e}"));
        }

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
        
        let command = Command::perform(check_for_updates(), MsgGlobal::CheckedForUpdate).map(Msg::Global);

        (Self {
            home_dir,
            app_root: flags.app_root,
            profiles,
            settings,
            active_scene: SceneType::HomePage(SceneHomePage {}),
            main_window_id: flags.main_window_id,
        }, command)
    }
    fn title(&self) -> String {
        "AcornGM".to_string()
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        if let Msg::Global(msg) = message {
            return self.handle_global_messages(msg).unwrap_or_else(|e| {
                show_error_dialogue("AcornGM Error while handling global message", &e);
                Command::none()
            });
        }

        let scene_ptr = &mut self.active_scene as *mut SceneType;
        let app = self;
        
        // safe because scene is not read or written to while pattern matching; only by scene.update()
        let result = unsafe {
            match &mut *scene_ptr {
                SceneType::HomePage(scene) => scene.update(app, message),
                SceneType::CreateProfile(scene) => scene.update(app, message),
                SceneType::ViewProfile(scene) => scene.update(app, message),
                SceneType::Login(scene) => scene.update(app, message),
            }
        };
        result.unwrap_or_else(|e| {
            show_error_dialogue("AcornGM Error while handling message", &e);
            Command::none()
        })
    }
    fn view(&self) -> Element<Self::Message> {
        let result = match &self.active_scene {
            SceneType::HomePage(scene) => scene.view(self),
            SceneType::CreateProfile(scene) => scene.view(self),
            SceneType::ViewProfile(scene) => scene.view(self),
            SceneType::Login(scene) => scene.view(self),
        };
        result.unwrap_or_else(|e| {
            text(format!("Error while rendering UI: {e}")).into()
        })
    }
    fn theme(&self) -> iced::Theme {
        iced::Theme::GruvboxDark
    }
    fn subscription(&self) -> Subscription<Msg> {
        if self.settings.access_token.is_none() && matches!(self.active_scene, SceneType::Login(_)) {
            return time::every(Duration::new(3, 0))
                .map(|_| Msg::Login(MsgLogin::SubRequestAccessToken))
        }
        Subscription::none()
        // time::every(Duration::new(10, 0))
        //     .map(|_| Msg::Global(MsgGlobal::CheckedForUpdate))
    }
}

impl MyApp {
    fn handle_global_messages(&mut self, message: MsgGlobal) -> Result<Command<Msg>, String> {
        match message {
            MsgGlobal::CheckedForUpdate(result) => if let Some(asset_file_url) = result? {
                let future = download_update_file(self.home_dir.clone(), asset_file_url);
                return Ok(Command::perform(future, MsgGlobal::DownloadedUpdateFile).map(Msg::Global))
            },
            MsgGlobal::DownloadedUpdateFile(result) => {
                result?;
                log::info!("dinner is ready")
            },
        }
        
        Ok(Command::none())
    }
}


pub fn main() -> iced::Result {
    biologischer_log::init(env!("CARGO_CRATE_NAME"));

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
    
    let icon: Option<Icon> = match iced::window::icon::from_file_data(APP_ICON, None) {
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
        id: Some("AcornGM".to_string()),
        window: window_settings,
        flags: MyAppFlags {
            main_window_id: iced::window::Id::unique(),
            app_root,
        },
        fonts: vec![],
        default_font: Font::DEFAULT,
        default_text_size: Pixels(14.0),
        antialiasing: true,
    };

    MyApp::run(settings)
}

