// SPDX-License-Identifier: EUPL-1.2

use crate::Message;
use crate::styles::button_unsupported::button_unsupported;
use crate::styles::tooltip_style::tooltip_content;
use iced::widget::button::{primary, secondary};
use iced::widget::tooltip::Position;
use iced::widget::{Container, button, container, text, tooltip};
use rust_i18n::t;
use tiny4linux::{AIMode, TrackingSpeed};

/// A tracking mode button.
///
/// `supported` is false for modes the connected camera does not have, which leaves
/// the button disabled instead of letting it look clickable and do nothing (#72).
pub fn button_tracking_mode(
    mode: AIMode,
    current_mode: AIMode,
    supported: bool,
) -> Container<'static, Message> {
    container(tooltip(
        button(text(format!("{}", mode.to_string())))
            .on_press_maybe(supported.then_some(Message::ChangeTracking(mode)))
            .style(if !supported {
                button_unsupported
            } else if current_mode == mode {
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

/// A tracking speed button, disabled on cameras without a tracking speed setting.
pub fn button_tracking_speed(
    speed: TrackingSpeed,
    current_speed: TrackingSpeed,
    supported: bool,
) -> Container<'static, Message> {
    container(tooltip(
        button(text(format!("{}", speed.to_string())))
            .on_press_maybe(supported.then_some(Message::ChangeTrackingSpeed(speed)))
            .style(if !supported {
                button_unsupported
            } else if current_speed == speed {
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
