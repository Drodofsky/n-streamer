use iced::{
    Element,
    Length::{self, FillPortion},
    widget::{column, container, row, slider, text, toggler},
};

use super::*;
impl NStreamer {
    pub(crate) fn view_sound_popup(&self) -> Element<'_, Message> {
        create_pop_up(
            "Audio".into(),
            container(
                column![
                    row![
                        text("volume"),
                        slider(0.0..=100.0, self.settings.volume(), |v| Message::Settings(
                            SettingsMessage::SetVolume(v)
                        ))
                    ]
                    .spacing(SPACING),
                    row![
                        text("muted").width(Length::Fill),
                        toggler(self.settings.muted())
                            .on_toggle(|b| Message::Settings(SettingsMessage::SetMuted(b)))
                    ]
                    .spacing(SPACING),
                    hl(1),
                    row![
                        text_button("ok")
                            .width(FillPortion(1))
                            .on_press(Message::Settings(SettingsMessage::SaveAndCloseSettings))
                    ]
                ]
                .padding(PADDING)
                .spacing(SPACING),
            ),
        )
        .into()
    }
}
