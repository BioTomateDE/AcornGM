use iced::{alignment, Color, Element};
use iced::widget::{button, column, container, row, scrollable, text, Column};
use crate::{GameType, Msg, SceneCreateProfile, SceneMain, SceneType};
use crate::utility::get_default_profile_path;

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
        let color_text1: Color = Color::from_rgb8(231, 227, 213);
        let color_text2: Color = Color::from_rgb8(147, 146, 145);

        let profiles: Column<Msg> = column(
            scene_homepage.profiles.iter().map(
                |i| i.view()
            )
        );

        let main_content = container(
            iced::widget::column![
                column![
                    text("").size(20),
                    text("GMAcorn").size(46).color(color_text1),
                ]
                .width(1000)
                .align_x(alignment::Horizontal::Center),
                column![
                    text("").size(20),
                    text("Recent Profiles").size(20).color(color_text2).align_x(alignment::Horizontal::Center),
                    scrollable(profiles).height(500),
                    // text("").size(18),
                ]
                .padding(20)
            ]
        );


        container(
            column![
                main_content,
                row![
                    button("Create Profile").on_press(Msg::HomePage(MsgHomePage::CreateProfile)),
                    button("Sample Text"),
                    button("Lorem ipsum"),
                ]
                .spacing(10)
            ]
                .width(900)
                .align_x(alignment::Horizontal::Right)
        )
            .into()
    }

    pub fn update_homepage(&mut self, _scene_homepage: &SceneHomePage, message: Msg) {
        match message {
            Msg::HomePage(MsgHomePage::CreateProfile) => {
                let default_profile_path: String = get_default_profile_path().unwrap_or_else(|error| {
                    println!("[WARN]  Could not get default profile path: {error}");
                    "".to_string()
                });

                self.active_scene = SceneType::CreateProfile(SceneCreateProfile {
                    profile_name: "Profile Name".to_string(),
                    profile_path: default_profile_path,
                    data_file_path: "".to_string(),
                    game_type: GameType::Unset,
                })
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

