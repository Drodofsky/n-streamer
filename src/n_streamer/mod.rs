mod live_stream;
mod message;
mod settings;
mod time;
use std::time::Duration;
mod config;
mod error;
mod program_schedule;
mod ui_utils;
mod utils;
use crate::{
    button_text,
    n_streamer::{
        config::Config, live_stream::LiveStream, program_schedule::ProgramSchedule,
        ui_utils::SPACING, utils::get_default_media_dir,
    },
    primary_text,
};
use iced::{
    Alignment, Element,
    Length::{self, Fill},
    Subscription, Task,
    widget::{column, container, row, stack, text},
    window,
};

use message::Message;

use crate::n_streamer::{
    settings::Settings,
    time::Time,
    ui_utils::{DynView, PADDING},
};

pub struct NStreamer {
    settings: Settings,
    time: Time,
    theme: iced::Theme,
    user_interaction: Option<DynView<Self, Message>>,
    life_stream: LiveStream,
    center: Center,
    program_schedule: ProgramSchedule,
    config: Config,
}

impl Default for NStreamer {
    fn default() -> Self {
        Self {
            theme: iced::Theme::Dark,
            settings: Settings::default(),
            time: Time::default(),
            user_interaction: None,
            life_stream: LiveStream::default(),
            center: Center::default(),
            program_schedule: ProgramSchedule::default(),
            config: Config::default(),
        }
    }
}

impl NStreamer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn init() -> (Self, Task<Message>) {
        let mut n_streamer = Self::new();
        let schedule = n_streamer.program_schedule.update_schedule();
        let config = Task::perform(Config::load(), Message::ConfigLoaded);

        let task = Task::batch([config, schedule]);
        (n_streamer, task)
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.time.update();
                let res = self.program_schedule.update_current_episode();
                self.apply_result(res);
                Task::none()
            }
            Message::ExitRequest(id) => {
                self.user_interaction = Some(Box::new(move |s| s.view_exit_popup(id)));
                Task::none()
            }
            Message::Exit(id) => window::close(id),
            Message::ClosePopUp => {
                self.user_interaction = None;
                Task::none()
            }
            Message::SettingSelected(m) => self.apply_settings_menu(m),
            Message::NewLiveStream(live_stream) => {
                let res = self.life_stream.new_live_stream(live_stream);
                self.apply_result(res);
                Task::none()
            }
            Message::MenuButtonPressed(Center::LiveStream) => {
                if let Some(url) = self.config.stream_url() {
                    self.center = Center::LiveStream;

                    self.life_stream.live_stream_button_pressed(url)
                } else {
                    self.user_interaction = Some(Box::new(|s| {
                        s.view_error_popup(
                            "Please configure a streaming url in settings.".to_string(),
                        )
                    }));
                    Task::none()
                }
            }
            Message::MenuButtonPressed(c) => {
                self.center = c;
                Task::none()
            }
            Message::NewSchedule(schedule) => {
                let res = self.program_schedule.new_schedule(schedule);
                self.apply_result(res);
                Task::none()
            }
            Message::ScheduleProgramSelected(program) => {
                self.program_schedule.select_episode(program);
                Task::none()
            }
            Message::ConfigLoaded(config) => {
                self.apply_result_and(config, Self::set_config);
                if self.config.media_path().is_none() {
                    self.apply_result_and(get_default_media_dir(), |s, path| {
                        s.config.set_media_path(path)
                    });
                }
                self.update_theme()
            }
            Message::UpdateTheme(theme) => {
                self.user_interaction = None;
                let t1 = self.config.set_theme(theme);

                let t2 = self.update_theme();
                Task::batch([t1, t2])
            }
            Message::ApplyTheme(theme) => {
                self.theme = theme;
                Task::none()
            }
            Message::Saved(result) => {
                self.apply_result(result);
                Task::none()
            }
            Message::NewStreamUrl(url) => {
                self.config.set_stream_url(url);
                Task::none()
            }
            Message::NewMediaPath(path) => {
                self.config.set_media_path(path.into());
                Task::none()
            }
            Message::MaybeNewMediaPath(path) => {
                if let Some(path) = path {
                    self.config.set_media_path(path.into());
                }
                Task::none()
            }
            Message::OpenMediaPathBrowser => {
                Task::perform(Settings::browse_media_path(), Message::MaybeNewMediaPath)
            }
            Message::SaveAndClosePopup => {
                self.user_interaction = None;
                Task::perform(Config::save(self.config.clone()), Message::Saved)
            }
        }
    }
    pub fn subscription(&self) -> Subscription<Message> {
        let tick = iced::time::every(Duration::from_millis(500)).map(|_| Message::Tick);
        let close = window::close_requests().map(Message::ExitRequest);
        Subscription::batch([tick, close])
    }
    pub fn view(&self) -> Element<'_, Message> {
        if let Some(interaction) = &self.user_interaction {
            let mut col = column![];
            col = col.push(self.view_top());
            col = col.push(stack([self.view_center(), (interaction(self))]));
            return col.into();
        }
        column![self.view_top(), self.view_center()].into()
    }
    fn view_current_program(&self) -> Element<'_, Message> {
        self.program_schedule
            .get_current_episode()
            .map(|e| primary_text!(e))
            .unwrap_or(primary_text!(""))
            .width(Fill)
            .into()
    }

    fn view_top(&self) -> Element<'_, Message> {
        container(
            row![
                self.settings.view(),
                button_text!("Program Schedule")
                    .on_press(Message::MenuButtonPressed(Center::ProgramSchedule)),
                button_text!("Watch Live").on_press(Message::MenuButtonPressed(Center::LiveStream)),
                self.view_current_program(),
                button_text!("Manage Downloads")
                    .on_press(Message::MenuButtonPressed(Center::Downloads)),
                button_text!("Library").on_press(Message::MenuButtonPressed(Center::Library)),
                self.time.view()
            ]
            .spacing(SPACING)
            .align_y(Alignment::Center),
        )
        .padding(PADDING)
        .style(container::bordered_box)
        .into()
    }
    fn view_center(&self) -> Element<'_, Message> {
        let center = match self.center {
            Center::LiveStream => self.life_stream.view(),
            Center::ProgramSchedule => self.program_schedule.view(),
            _ => text("Hello World!").into(),
        };
        container(center).center(Length::Fill).into()
    }

    pub fn theme(&self) -> iced::Theme {
        self.theme.clone()
    }
    fn set_config(&mut self, config: Config) {
        self.config = config;
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Center {
    #[default]
    ProgramSchedule,
    LiveStream,
    Downloads,
    Library,
}
