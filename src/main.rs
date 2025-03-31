mod scenes;
mod utility;

use iced::{Color, Element, Font, Pixels, Size};
use crate::scenes::create_profile::{MsgCreateProfile, SceneCreateProfile};
use crate::scenes::homepage::{MsgHomePage, SceneHomePage};
use iced::Settings;

#[derive(Debug, Clone)]
enum Msg {
    HomePage(MsgHomePage),
    CreateProfile(MsgCreateProfile),
}


#[derive(Debug, Clone)]
struct SceneMain {
    active_scene: SceneType,
    color_text1: Color,
    color_text2: Color,
}
impl Default for SceneMain {
    fn default() -> Self {
        SceneMain {
            active_scene: SceneType::default(),
            color_text1: Color::from_rgb8(231, 227, 213),
            color_text2: Color::from_rgb8(147, 146, 145),
        }
    }
}

impl SceneMain {
    fn update(&mut self, message: Msg) {
        let mut scene = std::mem::take(&mut self.active_scene); // Take ownership of `self.active_scene`

        self.active_scene = match &mut scene {
            SceneType::HomePage(scene) => self.update_homepage(scene, message),
            SceneType::CreateProfile(scene) => self.update_create_profile(scene, message),
        }
    }

    fn view(&self) -> Element<Msg> {
        match &self.active_scene {
            SceneType::HomePage(scene) => self.view_homepage(scene),
            SceneType::CreateProfile(scene) => self.view_create_profile(scene),
        }
    }
}


#[derive(Default, Debug, Clone)]
enum GameType {
    #[default]
    Unset,
    Undertale,
    Deltarune,
    Other,
}


#[derive(Debug, Clone)]
enum SceneType {
    HomePage(SceneHomePage),
    CreateProfile(SceneCreateProfile)
}
impl Default for SceneType {
    fn default() -> Self {
        SceneType::HomePage(SceneHomePage::default())
    }
}


pub fn main() -> iced::Result {
    let settings = Settings {
        id: Some("ts id pmo".to_string()),
        fonts: vec![],
        default_font: Font::DEFAULT,
        default_text_size: Pixels(14.0),
        antialiasing: true,
    };

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
            application_id: "idk what this application id is supposed to be".to_string(),
            override_redirect: false,
        },
        exit_on_close_request: true,
    };

    iced::application("GMAcorn", SceneMain::update, SceneMain::view)
        .theme(|_| iced::Theme::GruvboxDark)
        .settings(settings)
        .window(window_settings)
        .run()

}

