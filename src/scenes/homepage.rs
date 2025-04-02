use iced::{alignment, Element};
use iced::widget::{button, column, container, row, scrollable, text, Column};
use crate::{GameType, Msg, SceneCreateProfile, SceneMain, SceneType};
use crate::default_file_paths::get_default_profile_path;
use crate::utility::{get_default_icon_image};

#[derive(Debug, Clone)]
pub enum MsgHomePage {
    CreateProfile,
}


#[derive(Debug, Clone)]
pub struct SceneHomePage {
    pub profiles: Vec<Profile>,
    pub profiles_loading_state: ProfilesLoadingState,
}
impl Default for SceneHomePage {
    fn default() -> Self {
        SceneHomePage {
            profiles: vec![],
            profiles_loading_state: ProfilesLoadingState::NotLoaded,
        }
    }
}


impl SceneMain {
    pub fn view_homepage<'a>(&self, scene_homepage: &'a SceneHomePage) -> Element<'a, Msg> {
        let profiles: Column<Msg> = column(
            scene_homepage.profiles.iter().map(
                |i| i.view()
            )
        );

        let main_content = container(
            iced::widget::column![
                column![
                    text("").size(10),
                    text("AcornGM").size(28).color(self.color_text1),
                    text("").size(6),
                    text("Recent Profiles").size(12).color(self.color_text2).align_x(alignment::Horizontal::Center),
                    scrollable(profiles).height(100),
                    // text("").size(18),
                ]
                .padding(20)
            ]
        ).align_x(alignment::Horizontal::Left);

        let button_bar = container(
            row![
                button("Create Profile").on_press(Msg::HomePage(MsgHomePage::CreateProfile)),
                button("Sample Text"),
                button("Lorem ipsum"),
                text("    ").size(10)
            ]
                .spacing(10)
        )
            .width(900)
            .align_x(alignment::Horizontal::Right);

        container(
            column![
                column![
                    main_content,
                ]
                .height(460),
                button_bar
            ]
        )
            .into()
    }

    pub fn update_homepage(&mut self, message: Msg) {
        match message {
            Msg::HomePage(MsgHomePage::CreateProfile) => {
                let default_profile_path: String = get_default_profile_path().unwrap_or_else(|error| {
                    println!("[WARN]  Could not get default profile path: {error}");
                    "".to_string()
                });

                self.active_scene = SceneType::CreateProfile1(SceneCreateProfile {
                    profile_name: "My Profile".to_string(),
                    is_profile_name_valid: true,
                    icon: get_default_icon_image(),
                    profile_path: default_profile_path,
                    data_file_path: "".to_string(),
                    game_type: GameType::Unset,
                });
            },
            _ => {},
        }
    }
}


#[derive(Default, Debug, Clone)]
pub enum ProfilesLoadingState {
    #[default]
    NotLoaded,
    CurrentlyLoading,
    Loaded,
}

#[derive(Default, Debug, Clone)]
struct Profile {
    name: String,
    id: i32
}

impl Profile {
    fn view(&self) -> Element<Msg> {
        container(
            row![text(&self.name), text(self.id)]
        ).into()
    }
}

