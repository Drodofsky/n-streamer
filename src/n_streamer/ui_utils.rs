use chrono::TimeDelta;

pub const PADDING: u16 = 5;
pub const BIG_PADDING: u16 = 10;
pub const SPACING: u32 = 5;
pub type DynView<T, M> = Box<dyn Fn(&T) -> iced::Element<'_, M>>;
#[macro_export]
macro_rules! pop_up {
    ($message:expr , $action:expr) => {
        iced::widget::container(iced::widget::column![
            iced::widget::Space::new().height(iced::Length::FillPortion(2)),
            iced::widget::row![
                iced::widget::Space::new().width(iced::Length::FillPortion(1)),
                iced::widget::container(iced::widget::column![
                    iced::widget::container(iced::widget::text($message))
                        .padding($crate::n_streamer::ui_utils::PADDING),
                    iced::widget::container(iced::widget::rule::horizontal(2))
                        .padding($crate::n_streamer::ui_utils::PADDING),
                    $action
                ])
                .style(iced::widget::container::bordered_box)
                .padding($crate::n_streamer::ui_utils::PADDING),
                iced::widget::Space::new().width(iced::Length::FillPortion(1))
            ],
            iced::widget::Space::new().height(iced::Length::FillPortion(3))
        ])
        .center(iced::Length::Fill)
        .style(|theme| iced::widget::container::background(theme.palette().background))
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
        iced::widget::container(iced::widget::text($text).style(|theme| {
            iced::widget::text::Style {
                color: Some(
                    iced::widget::button::primary(theme, iced::widget::button::Status::Active)
                        .text_color,
                ),
            }
        }))
        .style(|theme| {
            let style = iced::widget::container::rounded_box(theme).background(
                iced::widget::button::primary(theme, iced::widget::button::Status::Active)
                    .background
                    .unwrap_or(iced::Background::Color(iced::Color::default())),
            );
            style
        })
        .padding($crate::n_streamer::ui_utils::BIG_PADDING)
        .align_x(iced::Center)
    };
}

pub fn fmt_period(period: &TimeDelta) -> String {
    let seconds = period.num_seconds();
    format!(
        "{:02}:{:02}:{:02}",
        seconds / 3600,
        (seconds % 3600) / 60,
        seconds % 60
    )
}
