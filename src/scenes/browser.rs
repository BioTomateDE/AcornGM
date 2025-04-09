use iced::{Color, Command, Element};
use iced::widget::{container, column, text_input, Container, row, button, text, checkbox};
use crate::Msg;
use crate::scenes::view_profile::{AcornMod, MsgViewProfile, SceneViewProfile};


#[derive(Debug, Clone)]
pub enum MsgBrowser {
    ToggleRegex,
    PerformSearch,
    EditSearchQuery(String),
    ToggleOnlyCompatible(bool),
}

#[derive(Default, Debug, Clone)]
pub struct ModBrowser {
    pub search_query: String,
    pub mods: Vec<AcornMod>,
    pub show_only_compatible: bool,
}
impl ModBrowser {
    pub fn view(&self, color_text1: Color, color_text2: Color) -> Element<Msg> {
        let search_bar: Container<Msg> = container(
            column![
                row![
                    row![
                        text_input("Search...", &self.search_query)
                            .on_input(|string| Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::EditSearchQuery(string))))
                            .on_submit(Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::PerformSearch))),
                        text("  ").size(10),
                        button(".*").on_press(Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::ToggleRegex))),      // TODO style: background diff when toggled off
                    ],
                    text("  ").size(10),
                    button("Search").on_press(Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::PerformSearch))),
                    text("  ").size(10),
                ],
                text("").size(6),
                checkbox("Only show compatible mods", self.show_only_compatible).on_toggle(|state| Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::ToggleOnlyCompatible(state)))),
                text("").size(8),
            ]
        );

        container(
            column![
                search_bar,
            ].spacing(6),
        )
            .into()
    }

    pub fn update(&mut self, message: MsgBrowser) -> Command<Msg> {
        match message {
            MsgBrowser::PerformSearch => {
                // stub
            },
            MsgBrowser::ToggleRegex => {
                // stub
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

