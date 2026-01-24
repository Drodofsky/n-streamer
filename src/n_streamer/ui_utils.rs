use super::*;
use iced::{Renderer, Theme, widget::Container};
pub const BIG_PADDING: u16 = 10;
pub fn primary_text<'s>(text: String) -> Container<'s, Message, Theme, Renderer> {
    iced::widget::container(
        iced::widget::text(text).style(|theme| iced::widget::text::Style {
            color: Some(
                iced::widget::button::primary(theme, iced::widget::button::Status::Active)
                    .text_color,
            ),
        }),
    )
    .style(|theme| {
        iced::widget::container::rounded_box(theme).background(
            iced::widget::button::primary(theme, iced::widget::button::Status::Active)
                .background
                .unwrap_or(iced::Background::Color(iced::Color::default())),
        )
    })
    .padding(BIG_PADDING)
    .align_x(iced::Center)
}
