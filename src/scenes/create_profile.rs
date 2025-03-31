use iced::Element;
use iced::widget::{container, column};
use crate::{GameType, Msg, SceneMain};


#[derive(Debug, Clone)]
pub enum MsgCreateProfile {

}

#[derive(Default, Debug, Clone)]
pub struct SceneCreateProfile {
    pub profile_name: String,
    pub profile_path: String,
    pub data_file_path: String,
    pub game_type: GameType,
}

impl SceneMain {
    pub fn update_create_profile(&mut self, scene_create_profile: &SceneCreateProfile, message: Msg) {

    }

    pub fn view_create_profile(&self, scene_create_profile: &SceneCreateProfile) -> Element<Msg> {
        container(
            column![]
        ).into()
    }
}

