mod view_profile1;
pub use view_profile1::MsgViewProfile;

use iced::advanced::image::Handle;
use crate::scenes::browser::ModBrowser;
use crate::scenes::homepage::Profile;
use crate::scenes::mod_details::ModDetails;
use crate::utility::{GameType, PlatformType, Version};


#[derive(Debug, Clone)]
pub struct SceneViewProfile {
    pub profile: Profile,
    pub mods: Vec<AcornMod>,
    pub browser: ModBrowser,
    pub mod_details: ModDetails,
}


#[derive(Debug, Clone)]
pub struct AcornMod {
    pub name: String,
    pub icon: Handle,
    pub author_name: String,
    pub mod_version: Version,
    pub date_created: chrono::DateTime<chrono::Local>,
    pub date_last_updated: chrono::DateTime<chrono::Local>,
    pub supported_games: Vec<GameType>,
    pub supported_game_versions: Vec<Version>,
    pub supported_platforms: Vec<PlatformType>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct AcornModLocal {
    pub acorn_mod: AcornMod,
    pub active: bool,           // whether the mod is enabled and will modify the game data
    pub filename: String,       // the name (uuid) of the mod without file extension or directory
}

impl Default for AcornMod {
    fn default() -> Self {
        Self {
            name: "Unknown Mod".to_string(),
            icon: Handle::from_pixels(256, 256, vec![]),        //TODO check if ts works
            author_name: "Unknown Mod Author".to_string(),
            mod_version: Default::default(),
            date_created: Default::default(),
            date_last_updated: Default::default(),
            supported_games: vec![],
            supported_game_versions: vec![],
            supported_platforms: vec![],
            description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string(),
        }
    }
}

