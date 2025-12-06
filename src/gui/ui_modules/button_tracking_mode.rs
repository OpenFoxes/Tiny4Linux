// SPDX-License-Identifier: EUPL-1.2

use crate::Message;
use crate::styles::tooltip_style::tooltip_content;
use iced::widget::button::{primary, secondary};
use iced::widget::tooltip::Position;
use iced::widget::{Container, button, container, text, tooltip};
use rust_i18n::t;
use tiny4linux::{AIMode, TrackingSpeed};

pub fn button_tracking_mode(mode: AIMode, current_mode: AIMode) -> Container<'static, Message> {
    container(tooltip(
        button(text(format!("{}", mode.to_string())))
            .on_press(Message::ChangeTracking(mode))
            .style(if current_mode == mode {
                primary
            } else {
                secondary
            }),
        tooltip_content(container(text(t!(
            "gui.tooltips.sets_tracking_mode",
            mode = mode.to_string()
        )))),
        Position::Bottom,
    ))
}

pub fn button_tracking_speed(
    speed: TrackingSpeed,
    current_speed: TrackingSpeed,
) -> Container<'static, Message> {
    container(tooltip(
        button(text(format!("{}", speed.to_string())))
            .on_press(Message::ChangeTrackingSpeed(speed))
            .style(if current_speed == speed {
                primary
            } else {
                secondary
            }),
        tooltip_content(container(text(t!(
            "gui.tooltips.sets_tracking_speed",
            speed = speed.to_string()
        )))),
        Position::Bottom,
    ))
}
