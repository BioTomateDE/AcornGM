mod scenes;
mod utility;
mod default_file_paths;

use iced::{Color, Element, Font, Pixels, Sandbox, Size};
use crate::scenes::create_profile1::{MsgCreateProfile1, SceneCreateProfile};
use crate::scenes::homepage::{MsgHomePage, SceneHomePage};
use iced::Settings;
use crate::scenes::create_profile2::MsgCreateProfile2;

#[derive(Debug, Clone)]
enum Msg {
    HomePage(MsgHomePage),
    CreateProfile1(MsgCreateProfile1),
    CreateProfile2(MsgCreateProfile2),
}


#[derive(Debug, Clone)]
struct SceneMain {
    active_scene: SceneType,
    color_text1: Color,
    color_text2: Color,
    color_text_red: Color,
}
impl Default for SceneMain {
    fn default() -> Self {
        SceneMain {
            active_scene: SceneType::default(),
            color_text1: Color::from_rgb8(231, 227, 213),
            color_text2: Color::from_rgb8(147, 146, 145),
            color_text_red: Color::from_rgb8(237, 49, 31),
        }
    }
}

impl SceneMain {
    fn update(&mut self, message: Msg) {
        match &self.active_scene {
            SceneType::HomePage(_) => self.update_homepage(message),
            SceneType::CreateProfile1(_) => self.update_create_profile1(message),
            SceneType::CreateProfile2(_) => self.update_create_profile2(message),
        }
    }

    fn view(&self) -> Element<Msg> {
        match &self.active_scene {
            SceneType::HomePage(scene) => self.view_homepage(scene),
            SceneType::CreateProfile1(scene) => self.view_create_profile1(scene),
            SceneType::CreateProfile2(scene) => self.view_create_profile2(scene),
        }
    }
}

#[derive(Debug, Clone)]
struct MyApp {
    scene_main: SceneMain,
}

impl Sandbox for MyApp {
    type Message = Msg;
    fn new() -> Self {
        Self { scene_main: Default::default() }
    }
    fn title(&self) -> String {
        "AcornGM".to_string()
    }
    fn theme(&self) -> iced::Theme {
        iced::Theme::GruvboxDark
    }
    fn update(&mut self, message: Self::Message) {
        self.scene_main.update(message)
    }
    fn view(&self) -> iced::Element<Self::Message> {
        self.scene_main.view()
    }
}


#[derive(Debug, Clone)]
enum SceneType {
    HomePage(SceneHomePage),
    CreateProfile1(SceneCreateProfile),
    CreateProfile2(SceneCreateProfile),
}
impl Default for SceneType {
    fn default() -> Self {
        SceneType::HomePage(SceneHomePage::default())
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

    let settings = Settings {
        id: Some("ts id pmo".to_string()),
        window: window_settings,
        flags: (),
        fonts: vec![],
        default_font: Font::DEFAULT,
        default_text_size: Pixels(14.0),
        antialiasing: true,
    };

    MyApp::run(settings)
}

