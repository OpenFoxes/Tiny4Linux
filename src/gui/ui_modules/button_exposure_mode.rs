// SPDX-License-Identifier: EUPL-1.2

use crate::Message;
use crate::styles::tooltip_style::tooltip_content;
use iced::widget::button::secondary;
use iced::widget::tooltip::Position;
use iced::widget::{Container, button, container, text, tooltip};
use rust_i18n::t;
use tiny4linux::ExposureMode;

pub fn button_exposure_mode(mode: ExposureMode) -> Container<'static, Message> {
    container(tooltip(
        button(text(format!("{}", mode)))
            .on_press(Message::ChangeExposure(mode))
            .style(secondary),
        tooltip_content(container(text(t!(
            "gui.tooltips.changes_exposure",
            mode = mode
        )))),
        Position::Bottom,
    ))
}
