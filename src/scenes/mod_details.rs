use iced::{Color, Command, Element};
use iced::widget::{container, column, row, text, Image};
use crate::Msg;
use crate::scenes::view_profile::AcornMod;

#[derive(Debug, Clone)]
pub enum MsgModDetails {}

#[derive(Default, Debug, Clone)]
pub struct ModDetails {
    pub acorn_mod: Option<AcornMod>,
}
impl ModDetails {
    pub fn view(&self, color_text1: Color, color_text2: Color) -> Element<Msg> {
        if self.acorn_mod.is_none() {
            return container(
                column![
                    text("Select a mod from the browser or from your profile's mod list").size(24).style(color_text2),
                ],
            )
                .height(400)
                .into()
        }

        let acorn_mod: &AcornMod = self.acorn_mod.as_ref().unwrap();    // .unwrap() is ok because we checked if it's None, the function returned already
        let icon = Image::new(acorn_mod.icon.clone());

        container(
            column![
                row![
                    icon,   // height and width should be capped beforehand
                    text(&acorn_mod.name).size(24).style(color_text1),
                ],
                text("").size(6),
                row![
                    text("Author").size(12).style(color_text2).width(80),
                    text(&acorn_mod.author_name).size(16).style(color_text1),
                ],
                row![
                    text("Mod Version").size(12).style(color_text2).width(80),
                    text(acorn_mod.mod_version.to_string()).size(16).style(color_text1),
                ],
            ]
        )
            .height(400)
            .into()
    }

    pub fn update(&mut self, message: MsgModDetails) -> Command<Msg> {
        // match message {
        //     MsgBrowser::PerformSearch => {
        //         // stub
        //     },
        //
        //     MsgBrowser::ToggleRegex(use_regex) => {
        //         self.use_regex = use_regex;
        //     },
        //
        //     MsgBrowser::EditSearchQuery(string) => {
        //         self.search_query = string;
        //     },
        //
        //     MsgBrowser::ToggleOnlyCompatible(show_only_compatible) => {
        //         self.show_only_compatible = show_only_compatible;
        //     },
        // }

        Command::none()
    }
}

