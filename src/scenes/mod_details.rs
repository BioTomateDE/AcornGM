use iced::{Color, Command, Element};
use iced::widget::{container, column, text_input, Container, row, button, text, checkbox, scrollable};
use crate::Msg;
use crate::scenes::view_profile::AcornMod;

#[derive(Debug, Clone)]
pub enum MsgModDetails {
}

#[derive(Debug, Clone)]
pub struct ModDetails {
    pub acorn_mod: AcornMod,
}
impl ModDetails {
    pub fn view(&self, color_text1: Color, color_text2: Color) -> Element<Msg> {
        column![].into()
        // container(
        //     column![
        //         text("").size(8),
        //         row![
        //             row![
        //                 text_input("Search...", &self.search_query)
        //                     .on_input(|string| Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::EditSearchQuery(string))))
        //                     .on_submit(Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::PerformSearch))),
        //                 text("  ").size(10),
        //             ],
        //             text("  ").size(10),
        //             button("Search").on_press(Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::PerformSearch))),
        //             text("  ").size(10),
        //         ],
        //         text("").size(6),
        //         checkbox("Use Regex for search", self.use_regex)
        //             .on_toggle(|state| Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::ToggleRegex(state)))),
        //         text("").size(4),
        //         checkbox("Only show compatible mods", self.show_only_compatible)
        //             .on_toggle(|state| Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::ToggleOnlyCompatible(state)))),
        //         text("").size(10),
        //     ]
        // )
        //     .height(400)
        //     .into()
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

