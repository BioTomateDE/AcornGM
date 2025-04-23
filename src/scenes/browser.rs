use iced::{ Command, Element};
use iced::widget::{container, column, text_input, Container, row, button, text, checkbox, scrollable, Space};
use crate::Msg;
use crate::scenes::view_profile::{AcornMod, MsgViewProfile};
use crate::ui_templates::list_style;

#[derive(Debug, Clone)]
pub enum MsgBrowser {
    ToggleRegex(bool),
    PerformSearch,
    EditSearchQuery(String),
    ToggleOnlyCompatible(bool),
}

#[derive(Default, Debug, Clone)]
pub struct ModBrowser {
    pub search_query: String,
    pub use_regex: bool,
    pub results: Vec<AcornMod>,
    pub show_only_compatible: bool,
}
impl ModBrowser {
    pub fn view(&self) -> Element<Msg> {
        let results: Container<Msg> = container(
            column(
                self.results.iter().map(|i| i.view())
            ).spacing(5)
        )
            .width(415)
            .style(list_style);

        let search_bar: Container<Msg> = container(
            column![
                text("").size(8),
                row![
                    row![
                        text_input("Search...", &self.search_query)
                            .on_input(|string| Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::EditSearchQuery(string))))
                            .on_submit(Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::PerformSearch))),
                        Space::with_width(4.0),
                    ],
                    Space::with_width(4.0),
                    button("Search").on_press(Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::PerformSearch))),
                    Space::with_width(4.0),
                ],
                Space::with_height(7.0),
                checkbox("Use Regex for search", self.use_regex)
                    .on_toggle(|state| Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::ToggleRegex(state)))),
                Space::with_height(5.0),
                checkbox("Only show compatible mods", self.show_only_compatible)
                    .on_toggle(|state| Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::ToggleOnlyCompatible(state)))),
                Space::with_height(7.0),
            ]
        ).height(100);

        container(
            column![
                search_bar,
                scrollable(results).height(500),
                text("test")
            ].spacing(6),
        )
            .into()
    }

    pub fn update(&mut self, message: MsgBrowser) -> Command<Msg> {
        match message {
            MsgBrowser::PerformSearch => {
                // stub
            },

            MsgBrowser::ToggleRegex(use_regex) => {
                self.use_regex = use_regex;
            },

            MsgBrowser::EditSearchQuery(string) => {
                self.search_query = string;
            },

            MsgBrowser::ToggleOnlyCompatible(show_only_compatible) => {
                self.show_only_compatible = show_only_compatible;
            },
        }

        Command::none()
    }
}

