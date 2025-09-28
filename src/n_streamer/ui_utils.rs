use iced::Element;

pub const PADDING: u16 = 5;
pub const SPACING: u16 = 5;
pub type DynView<T, M> = Box<dyn Fn(&T) -> Element<'_, M>>;
#[macro_export]
macro_rules! pop_up {
    ($element:expr) => {
        container(column![
            Space::with_height(Length::FillPortion(2)),
            row![
                Space::with_width(Length::FillPortion(1)),
                container($element).style(container::bordered_box),
                Space::with_width(Length::FillPortion(1))
            ],
            Space::with_height(Length::FillPortion(3))
        ])
        .center(Length::Fill)
        .style(|theme| container::background(theme.palette().background))
    };
}

#[macro_export]
macro_rules! button_text {
    ($text:expr) => {
        button(text($text).align_x(Center))
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
                button::primary(theme, Status::Active)
                    .background
                    .unwrap_or(Background::Color(Color::default())),
            );
            style
        })
        .padding(PADDING)
    };
}
