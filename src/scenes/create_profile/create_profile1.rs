use std::path::PathBuf;
use iced::{alignment, Command, Element};
use iced::widget::{container, column, text, button, Image, text_input};
use iced::widget::image::Handle;
use log::{error, info, warn};
use rfd::FileDialog;
use crate::{Msg, MyApp, SceneType, COLOR_TEXT1, COLOR_TEXT2, COLOR_TEXT_RED};
use crate::scenes::homepage::SceneHomePage;
use crate::default_file_paths::get_default_image_prompt_path;
use crate::scenes::create_profile::{check_profile_name_valid, SceneCreateProfile};
use crate::ui_templates::generate_button_bar;
use crate::utility::show_error_dialogue;

#[derive(Debug, Clone)]
pub enum MsgCreateProfile1 {
    BackToHomepage,
    StepNext,
    EditProfileName(String),
    EditProfileIcon,
    PickedProfileIcon(Option<PathBuf>),
}

impl SceneCreateProfile {
    pub fn update1(&mut self, app: &mut MyApp, message: Msg) -> Command<Msg> {
        let message: MsgCreateProfile1 = match message {
            Msg::CreateProfile1(msg) => msg,
            other => {
                error!("Invalid message type {other:?}");
                return Command::none()
            }
        };

        match message {
            MsgCreateProfile1::BackToHomepage => {
                app.active_scene = SceneType::HomePage(SceneHomePage {});
            }
            MsgCreateProfile1::StepNext => {
                if self.is_file_picker_open {
                    show_error_dialogue("AcornGM User Error", "Please close the file picker before changing scene.");
                    return Command::none()
                }
                if self.is_profile_name_valid {
                    self.stage = 2;
                }
            }
            MsgCreateProfile1::EditProfileName(profile_name) => {
                self.is_profile_name_valid = check_profile_name_valid(&profile_name);
                self.profile_name = profile_name;
            }
            MsgCreateProfile1::EditProfileIcon => {
                if !self.is_file_picker_open {
                    self.is_file_picker_open = true;
                    return self.pick_profile_icon_image(app)
                }
            }
            MsgCreateProfile1::PickedProfileIcon(image_path) => {
                self.is_file_picker_open = false;
                self.set_icon_image(image_path);
            }
        }
        Command::none()
    }

    pub fn view1(&self, _app: &MyApp) -> Element<Msg> {
        let profile_name_valid = text(
            if self.is_profile_name_valid {""} else {"Invalid Profile Name"}
        ).size(12).style(*COLOR_TEXT_RED);

        let icon: Image<Handle> = Image::new(self.icon.clone());

        let main_content = container(
            iced::widget::column![
                column![
                    text("Create New Profile").size(22).style(*COLOR_TEXT1),
                    text("").size(10),
                    text("Profile Name").size(14).style(*COLOR_TEXT2),
                    text("").size(4),
                    text_input(
                        "My Profile",
                        &self.profile_name,
                    ).on_input(|string| Msg::CreateProfile1(MsgCreateProfile1::EditProfileName(string))),
                    text("").size(4),
                    profile_name_valid,
                    text("").size(10),
                    text("Profile Icon").size(14).style(*COLOR_TEXT2),
                    text("").size(4),
                    button(icon.height(100)).on_press(Msg::CreateProfile1(MsgCreateProfile1::EditProfileIcon)),
                ]
                .padding(20)
            ]
        ).align_x(alignment::Horizontal::Left);

        let button_bar = generate_button_bar(vec![
            button("Cancel").on_press(Msg::CreateProfile1(MsgCreateProfile1::BackToHomepage)).into(),
        ], vec![
            button("Next >").on_press(Msg::CreateProfile1(MsgCreateProfile1::StepNext)).into(),
        ]);
        
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
}

impl SceneCreateProfile {
    fn pick_profile_icon_image(&mut self, app: &MyApp) -> Command<Msg> {
        let origin_path: PathBuf = get_default_image_prompt_path().unwrap_or_else(|e| {
            warn!("Could not get default image prompt path: {e}");
            app.app_root.clone()
        });

        let file_dialog: FileDialog = FileDialog::new()
            .set_title("Pick an image for your AcornGM Profile icon")
            .add_filter("Image", &["png", "jpg", "jpeg", "webp", "gif"])
            .set_directory(origin_path);

        Command::perform(
            async move { file_dialog.pick_file() },
            |i| Msg::CreateProfile1(MsgCreateProfile1::PickedProfileIcon(i))
        )
    }

    fn set_icon_image(&mut self, image_path: Option<PathBuf>) {
        let image_path: PathBuf = match image_path {
            Some(path) => path,
            None => {
                info!("User did not pick an image file for the profile icon and instead cancelled the operation.");
                return;
            }
        };

        if !image_path.is_file() {
            warn!("Specified image path for profile icon doesn't exist: {}", image_path.display());
            return;
        }

        // success, set profile icon
        self.icon = Handle::from_path(image_path);
    }
}

