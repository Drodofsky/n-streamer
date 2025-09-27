pub const PADDING: u16 = 6;
pub const SPACING: u16 = 6;
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
macro_rules! text_btn {
    ($text:expr) => {
        button(text($text).align_x(Center))
    };
}
