use std::fmt;

use iced::{Background, Element, Task, widget::pick_list, window};

use crate::n_streamer::{NStreamer, message::Message};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuItem {
    Exit,
    Todo,
}

impl fmt::Display for MenuItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MenuItem::Exit => write!(f, "Exit"),
            MenuItem::Todo => write!(f, "TODO"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Menu;

impl Menu {
    pub fn view(&self) -> Element<'_, Message> {
        let options = [MenuItem::Exit, MenuItem::Todo];
        let selected: Option<MenuItem> = None;
        pick_list(options, selected, Message::MenuSelected)
            .placeholder("Menu")
            .style(|theme, status| {
                let mut p = pick_list::default(theme, status);
                p.placeholder_color = theme.palette().text;
                p.background = Background::Color(theme.palette().primary);
                p
            })
            .into()
    }
}

impl NStreamer {
    pub(crate) fn apply_menu(&mut self, menu_item: MenuItem) -> Task<Message> {
        match menu_item {
            MenuItem::Exit => window::get_latest().map(|id| match id {
                Some(id) => Message::CloseRequest(id),
                None => Message::Tick,
            }),
            MenuItem::Todo => {
                todo!()
            }
        }
    }
}
