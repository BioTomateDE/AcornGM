mod scenes;
mod utility;

use iced::Element;
use crate::scenes::create_profile::{MsgCreateProfile, SceneCreateProfile};
use crate::scenes::homepage::{MsgHomePage, SceneHomePage};

#[derive(Debug, Clone)]
enum Msg {
    HomePage(MsgHomePage),
    CreateProfile(MsgCreateProfile),
}


#[derive(Default, Debug, Clone)]
struct SceneMain {
    active_scene: SceneType,
}

impl SceneMain {
    fn update(&mut self, message: Msg) {
        let mut scene = std::mem::take(&mut self.active_scene); // Take ownership of `self.active_scene`

        match &mut scene {
            SceneType::HomePage(scene) => self.update_homepage(scene, message),
            SceneType::CreateProfile(scene) => self.update_create_profile(scene, message),
        }

        self.active_scene = scene; // Put the scene back
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
    iced::run("GMAcorn", SceneMain::update, SceneMain::view)
}

