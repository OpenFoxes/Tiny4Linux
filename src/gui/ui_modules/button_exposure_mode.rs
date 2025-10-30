use crate::Message;
use crate::styles::tooltip_style::tooltip_content;
use iced::widget::button::secondary;
use iced::widget::tooltip::Position;
use iced::widget::{Container, button, container, text, tooltip};
use tiny4linux::ExposureMode;

pub fn button_exposure_mode(mode: ExposureMode) -> Container<'static, Message> {
    container(tooltip(
        button(text(format!("{}", mode)))
            .on_press(Message::ChangeExposure(mode))
            .style(secondary),
        tooltip_content(container(text(format!(
            "Changes the exposure mode to {} exposure",
            mode
        )))),
        Position::Bottom,
    ))
}
