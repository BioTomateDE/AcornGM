use iced::{Color, Command, Element};
use iced::widget::{container, column, text_input, Container, row, button, text, checkbox, scrollable};
use iced::widget::container::Appearance;
use crate::Msg;
use crate::scenes::homepage::list_style;
use crate::scenes::view_profile::{AcornMod, MsgViewProfile};
use crate::utility::TransparentButton;

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
    pub fn view(&self, color_text1: Color, color_text2: Color) -> Element<Msg> {
        let results: Container<Msg> = container(
            column(
                self.results.iter().map(|i| i.view(color_text1, color_text2))
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
                        text("  ").size(10),
                        // button(".*")
                        //     .on_press(Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::ToggleRegex)))
                        //     .style(iced::theme::Button::Custom(if self.regex {Box::new(ToggleButtonEnabled)} else {Box::new(ToggleButtonDisabled)})),
                    ],
                    text("  ").size(10),
                    button("Search").on_press(Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::PerformSearch))),
                    text("  ").size(10),
                ],
                text("").size(6),
                checkbox("Use Regex for search", self.use_regex)
                    .on_toggle(|state| Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::ToggleRegex(state)))),
                text("").size(4),
                checkbox("Only show compatible mods", self.show_only_compatible)
                    .on_toggle(|state| Msg::ViewProfile(MsgViewProfile::Browser(MsgBrowser::ToggleOnlyCompatible(state)))),
                text("").size(10),
            ]
        ).height(100);

        container(
            column![
                search_bar,
                scrollable(results).height(400),
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

// #[derive(Debug, Clone, Copy)]
// pub struct ToggleButtonEnabled;
// impl button::StyleSheet for ToggleButtonEnabled {
//     type Style = iced::Theme;
//     fn active(&self, _: &Self::Style) -> button::Appearance {
//         button::Appearance {
//             text_color: Default::default(),
//             background: Some(iced::Background::Color(Color::from_rgb8(26, 212, 42))),
//             border: Default::default(),
//             shadow: Default::default(),
//             shadow_offset: Default::default(),
//         }
//     }
// }
// #[derive(Debug, Clone, Copy)]
// pub struct ToggleButtonDisabled;
// impl button::StyleSheet for ToggleButtonDisabled {
//     type Style = iced::Theme;
//     fn active(&self, _: &Self::Style) -> button::Appearance {
//         button::Appearance {
//             text_color: Color::from_rgb8(231, 227, 213),       // text_color1
//             background: Some(iced::Background::Color(Color::from_rgb8(197, 39, 29))),
//             border: iced::Border {
//                 color: Color::from_rgb8(106, 11, 5),
//                 width: 1.0,
//                 radius: Default::default(),
//             },
//             shadow: Default::default(),
//             shadow_offset: Default::default(),
//         }
//     }
// }


