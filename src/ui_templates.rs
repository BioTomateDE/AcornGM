use iced::{alignment, Color, Element, Length};
use iced::widget::{container, row, text, Container, Row, Space};
use iced::widget::container::Appearance;
use crate::Msg;

fn divider_style(_theme: &iced::Theme) -> Appearance {
    Appearance {
        background: Some(iced::Background::Color(Color::from_rgb8(34, 33, 31))),
        border: iced::Border {
            color: Color::TRANSPARENT,                  // No border for divider
            width: 0.0,                                 // No actual border width
            radius: iced::border::Radius::from(0),  // No border radius
        },
        shadow: Default::default(),
        text_color: None,
    }
}
pub fn create_divider() -> Element<'static, Msg> {
    Container::new(text(""))
        .height(0.75)                   // Height of the divider
        .width(iced::Length::Fill)      // Full width
        .center_x()                     // Center horizontally
        .style(divider_style)
        .into()
}

pub fn item_style(_theme: &iced::Theme) -> Appearance {
    Appearance {
        text_color: None,
        background: None,
        border: iced::Border::default(),
        shadow: Default::default(),
    }
}
pub fn list_style(_theme: &iced::Theme) -> Appearance {
    Appearance {
        text_color: None,
        background: Some(iced::Background::Color(Color::from_rgb8(47, 47, 46))),
        border: iced::Border {
            color: Color::from_rgb8(34, 33, 31),
            width: 2.0,
            radius: iced::border::Radius::from([0.0, 0.0, 0.0, 0.0]),
        },
        shadow: Default::default(),
    }
}

pub fn generate_button_bar<'a>(
    left_buttons: Vec<Element<'a, Msg>>,
    right_buttons: Vec<Element<'a, Msg>>,
) -> Container<'a, Msg> {
    container(
        row![
            Row::new()
                .push(Space::with_width(8.0))
                .extend(left_buttons)
            .spacing(10)
            .align_items(alignment::Alignment::Start),
            
            Space::with_width(Length::Fill),
            
            Row::new()
                .extend(right_buttons)
                .push(Space::with_width(8.0))
            .spacing(10)
            .align_items(alignment::Alignment::End),
        ]
            .width(Length::Fill)
    )
}

