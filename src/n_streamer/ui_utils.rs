use super::*;
use iced::widget::button::Status as ButtonStatus;
use iced::widget::pick_list::Status as PickListStatus;
use iced::widget::pick_list::Style as PickListStyle;
use iced::{Color, Renderer, Theme, widget::Container};
pub type DynView<T, M> = Box<dyn Fn(&T) -> iced::Element<'_, M>>;
pub const PADDING: u16 = 5;
pub const BIG_PADDING: u16 = 10;
pub const SPACING: u32 = 5;
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
pub fn text_button(text: &str) -> iced::widget::Button<'_, Message, Theme, Renderer> {
    iced::widget::button(iced::widget::text(text).align_x(iced::Alignment::Center))
}

pub fn create_pop_up(
    message: String,
    action: Container<'_, Message, Theme, Renderer>,
) -> Container<'_, Message, Theme, Renderer> {
    iced::widget::container(iced::widget::column![
        iced::widget::Space::new().height(iced::Length::FillPortion(2)),
        iced::widget::row![
            iced::widget::Space::new().width(iced::Length::FillPortion(1)),
            iced::widget::container(iced::widget::column![
                iced::widget::container(iced::widget::text(message)).padding(PADDING),
                hl(2).padding(PADDING),
                action
            ])
            .style(iced::widget::container::bordered_box)
            .padding(PADDING),
            iced::widget::Space::new().width(iced::Length::FillPortion(1))
        ],
        iced::widget::Space::new().height(iced::Length::FillPortion(3))
    ])
    .center(iced::Length::Fill)
    .style(|theme| iced::widget::container::background(theme.palette().background))
}

pub fn hl(size: u32) -> Container<'static, Message, Theme, Renderer> {
    iced::widget::container(iced::widget::rule::horizontal(size))
}

pub fn to_pick_list_style(button_style: iced::widget::button::Style) -> PickListStyle {
    PickListStyle {
        text_color: button_style.text_color,
        placeholder_color: button_style.text_color,
        background: button_style
            .background
            .unwrap_or(iced::Background::Color(Color::default())),
        border: button_style.border,
        handle_color: button_style.text_color,
    }
}

pub fn to_button_status(status: PickListStatus) -> ButtonStatus {
    match status {
        PickListStatus::Active => ButtonStatus::Active,
        PickListStatus::Hovered => ButtonStatus::Hovered,
        PickListStatus::Opened { .. } => ButtonStatus::Pressed,
    }
}
