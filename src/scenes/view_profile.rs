mod view_profile1;

pub use view_profile1::MsgViewProfile;
use std::path::PathBuf;
use chrono::{DateTime, Local};
use iced::advanced::image::Handle;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::resources::PLACEHOLDER_ICON;
use crate::scenes::browser::ModBrowser;
use crate::scenes::homepage::Profile;
use crate::scenes::mod_details::ModDetails;
use crate::utility::{PlatformType, GameVersion};


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
    pub icon: Handle,       // TODO implement this in the database
    pub author_name: String,
    pub mod_version: GameVersion,
    pub date_created: DateTime<Local>,
    pub date_last_updated: DateTime<Local>,
    pub target_game: String,
    pub target_game_version: GameVersion,
    pub supported_platforms: Vec<PlatformType>,
    pub description: String,
    // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcornModLocal {
    pub active: bool,           // whether the mod is enabled and will modify the game data
    pub reference: ModReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModReference {
    Online(Uuid),       // stored on AcornGM servers (can be cached)
    Local(PathBuf),     // stored locally
}

impl Default for AcornMod {
    fn default() -> Self {
        Self {
            name: "Unknown Mod".to_string(),
            icon: Handle::from_memory(PLACEHOLDER_ICON),        //TODO check if ts works
            author_name: "Unknown Mod Author".to_string(),
            mod_version: Default::default(),
            date_created: Default::default(),
            date_last_updated: Default::default(),
            target_game: "Unknown Game".to_string(),
            target_game_version: GameVersion::new(0, 0),
            supported_platforms: vec![],
            description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
                          Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string(),
        }
    }
}

