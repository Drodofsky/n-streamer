use chrono::TimeDelta;
use iced::Renderer;

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
                    $crate::hl!(2).padding($crate::n_streamer::ui_utils::PADDING),
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

#[macro_export]
macro_rules! hl {
    () => {
        iced::widget::container(iced::widget::rule::horizontal(1))
    };
    ($size:expr) => {
        iced::widget::container(iced::widget::rule::horizontal($size))
    };
}

pub trait Str {
    fn get_str(&self) -> String;
}
pub trait ScrollListMessage<Item: Str> {
    fn plus(owner: ScrollListOwner, item: Item) -> Self;
    fn list_element_entered(owner: ScrollListOwner, id: usize) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub enum ScrollListOwner {
    ProgramSchedule,
    DownloadQueue,
}

pub fn view_scroll_list<'s, Message: ScrollListMessage<Item> + Clone + 's, Item: Str + Clone>(
    items: &'s [Item],
    hovered_id: usize,
    owner: ScrollListOwner,
    text_button: &'s str,
) -> iced::widget::Scrollable<'s, Message, iced::Theme, Renderer> {
    let episodes = items
        .iter()
        .enumerate()
        .fold(iced::widget::Column::new(), |col, (id, item)| {
            col.push(
                iced::widget::mouse_area(
                    iced::widget::container(
                        iced::widget::row![
                            iced::widget::text(item.get_str()).style(move |theme: &iced::Theme| {
                                if hovered_id == id {
                                    let mut style = iced::widget::text::default(theme);
                                    style.color =
                                        Some(theme.extended_palette().background.strong.text);
                                    style
                                } else {
                                    iced::widget::text::default(theme)
                                }
                            }),
                            iced::widget::space().width(iced::Fill),
                            button_text!(text_button)
                                .style(move |theme, status| {
                                    let mut style = iced::widget::button::text(theme, status);
                                    if hovered_id == id {
                                        style.text_color =
                                            theme.extended_palette().background.strong.text;
                                    }
                                    style
                                })
                                .on_press(Message::plus(owner, item.clone()))
                        ]
                        .padding(PADDING)
                        .spacing(SPACING),
                    )
                    .style(move |theme: &iced::Theme| {
                        if hovered_id == id {
                            iced::widget::container::transparent(theme)
                                .background(theme.extended_palette().background.strong.color)
                        } else {
                            iced::widget::container::transparent(theme)
                        }
                    }),
                )
                .on_enter(Message::list_element_entered(owner, id)),
            )
        });
    iced::widget::scrollable(episodes.padding(PADDING).width(iced::Fill))
}
