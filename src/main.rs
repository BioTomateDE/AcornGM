mod scenes;
mod utility;

use iced::{Color, Element, Font, Pixels, Size};
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
    CreateProfile1(SceneCreateProfile),
    CreateProfile2(SceneCreateProfile),
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

