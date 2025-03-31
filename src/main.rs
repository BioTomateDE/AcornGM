use std::env;
use iced::{alignment, Color, Element, Length, Theme};
use iced::widget::{button, column, container, row, scrollable, text, Column};

#[derive(Debug, Clone)]
enum Message {
    CreateProfile,
}

#[derive(Default, Debug, Clone)]
enum ProfilesLoadingState {
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
    fn view(&self) -> Element<Message> {
        container(
            row![text(&self.name), text(self.id)]
        ).into()
    }
}


#[derive(Debug, Clone)]
struct SceneHomePage {
    profiles: Vec<Profile>,
    profiles_loading_state: ProfilesLoadingState,
}

#[derive(Default, Debug, Clone)]
struct SceneMain {
    active_scene: SceneType,
}

impl SceneMain {
    fn update(&mut self, message: Message) {
        let mut scene = std::mem::take(&mut self.active_scene); // Take ownership of `self.active_scene`

        match &mut scene {
            SceneType::HomePage(scene) => self.update_homepage(scene, message),
            SceneType::CreateProfile(scene) => self.update_create_profile(scene, message),
        }

        self.active_scene = scene; // Put the scene back
    }


    fn view(&self) -> Element<Message> {
        match &self.active_scene {
            SceneType::HomePage(scene) => self.view_homepage(scene),
            SceneType::CreateProfile(scene) => self.view_create_profile(scene),
        }
    }

    fn view_homepage<'a>(&self, scene_homepage: &'a SceneHomePage) -> Element<'a, Message> {
        let color_text1: Color = Color::from_rgb8(231, 227, 213);
        let color_text2: Color = Color::from_rgb8(147, 146, 145);

        let profiles: Column<Message> = column(
            scene_homepage.profiles.iter().map(
                |i| i.view()
            )
        );

        let main_content = container(
            column![
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
                    button("Create Profile").on_press(Message::CreateProfile),
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
}

fn view_profiles(profiles: &[Profile]) -> Element<Message> {
    container(
        column(
            profiles.iter().map(|i| i.view())
        )
    ).into()
}


#[derive(Default, Debug, Clone)]
enum GameType {
    #[default]
    Unset,
    Undertale,
    Deltarune,
    Other,
}

#[derive(Default, Debug, Clone)]
struct SceneCreateProfile {
    profile_name: String,
    profile_path: String,
    data_file_path: String,
    game_type: GameType,
}


#[derive(Debug, Clone)]
enum SceneType {
    HomePage(SceneHomePage),
    CreateProfile(SceneCreateProfile)
}
impl Default for SceneType {
    fn default() -> Self {
        SceneType::HomePage(SceneHomePage {
            profiles: vec![],
            profiles_loading_state: ProfilesLoadingState::NotLoaded,
        })
    }
}

// #[derive(Default, Debug, Clone)]
// struct Scene {
//     scene: SceneType,
// }
//
// impl Scene {
//     fn update(&mut self, message: Message) {
//         match &self.scene {
//             SceneType::HomePage(scene) => SceneHomePage::update(&mut scene),
//             SceneType::CreateProfile(scene) => scene.update(),
//         }
//     }
// }


fn get_default_profile_path() -> Result<String, String> {
    let username: String = whoami::username();
    if username == "" {
        return Err("Username returned by whoami::username() is empty.".to_string());
    }

    match env::consts::OS {
        "windows" => Ok(format!("C:/Users/{username}/Documents/GMAcorn/")),
        "linux" => Ok(format!("/home/{username}/Documents/GMAcorn/")),
        // "macos" => Ok(format!("unknown? i don't use macOS")),
        other => Err(format!("Unknown or unsupported operating system \"{other}\".")),
    }
    // println!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n", whoami::username(), whoami::arch(), whoami::desktop_env(), whoami::devicename(), whoami::platform(), whoami::realname(), whoami::distro());
    // "stub".to_string()
}


impl SceneMain {
    fn update_homepage(&mut self, scene_homepage: &SceneHomePage, message: Message) {
        match message {
            Message::CreateProfile => {
                let default_profile_path: String = get_default_profile_path().unwrap_or_else(|error| {
                    println!("Could not get default profile path: {error}");
                    "".to_string()
                });
                dbg!(&default_profile_path);

                self.active_scene = SceneType::CreateProfile(SceneCreateProfile {
                    profile_name: "Profile Name".to_string(),
                    profile_path: default_profile_path,
                    data_file_path: "".to_string(),
                    game_type: GameType::Unset,
                })
            }
        }
    }

    fn update_create_profile(&mut self, scene_create_profile: &SceneCreateProfile, message: Message) {

    }

    fn view_create_profile(&self, scene_create_profile: &SceneCreateProfile) -> Element<Message> {
        container(
            column![]
        ).into()
    }
}


pub fn main() -> iced::Result {
    iced::run("GMAcorn", SceneMain::update, SceneMain::view)
}

