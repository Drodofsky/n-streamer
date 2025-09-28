use iced::Element;

pub const PADDING: u16 = 6;
pub const SPACING: u32 = 6;
pub type DynView<T, M> = Box<dyn Fn(&T) -> Element<'_, M>>;
#[macro_export]
macro_rules! pop_up {
    ($element:expr) => {
        container(column![
            Space::new().width(Length::FillPortion(2)),
            row![
                Space::new().width(Length::FillPortion(1)),
                container($element).style(container::bordered_box),
                Space::new().width(Length::FillPortion(1))
            ],
            Space::new().height(Length::FillPortion(3))
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
