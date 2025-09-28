use iced::Element;

pub const PADDING: u16 = 5;
pub const SPACING: u32 = 5;
pub type DynView<T, M> = Box<dyn Fn(&T) -> Element<'_, M>>;
#[macro_export]
macro_rules! pop_up {
    ($element:expr) => {
        widget::container(widget::column![
            widget::Space::new().height(iced::Length::FillPortion(2)),
            widget::row![
                widget::Space::new().width(iced::Length::FillPortion(1)),
                widget::container($element).style(widget::container::bordered_box),
                widget::Space::new().width(iced::Length::FillPortion(1))
            ],
            widget::Space::new().height(iced::Length::FillPortion(3))
        ])
        .center(iced::Length::Fill)
        .style(|theme| widget::container::background(theme.palette().background))
    };
}

#[macro_export]
macro_rules! button_text {
    ($text:expr) => {
        iced::widget::button(iced::widget::text($text).align_x(iced::Alignment::Center))
    };
}

#[macro_export]
macro_rules! primary_text {
    ($text:expr) => {
        widget::container(widget::text($text).style(|theme| Style {
            color: Some(button::primary(theme, Status::Active).text_color),
        }))
        .style(|theme| {
            let style = transparent(theme).background(
                widget::button::primary(theme, Status::Active)
                    .background
                    .unwrap_or(Background::Color(Color::default())),
            );
            style
        })
        .padding(PADDING)
        .align_x(iced::Center)
    };
}
