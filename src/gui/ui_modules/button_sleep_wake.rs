// SPDX-License-Identifier: EUPL-1.2

use crate::styles::tooltip_style::tooltip_content;
use crate::{Message, WindowMode};
use iced::widget::tooltip::Position;
use iced::widget::{Container, button, container, row, tooltip};
use iced_font_awesome::fa_icon_solid;
use tiny4linux::SleepMode;

pub fn button_sleep_wake(
    sleep_mode: SleepMode,
    window_mode: WindowMode,
) -> Container<'static, Message> {
    let (text_element_text, icon, tooltip_text, message) = match sleep_mode {
        SleepMode::Awake => (
            "Set to Sleep",
            fa_icon_solid("moon"),
            "Request the camera to go to sleep mode",
            Message::ChangeSleeping(true),
        ),
        SleepMode::Sleep => (
            "Wake Up",
            fa_icon_solid("sun"),
            "Request the camera to wake up from sleep mode",
            Message::ChangeSleeping(false),
        ),
        SleepMode::Unknown => (
            "Sleep state unknown",
            fa_icon_solid("question"),
            "The state can't be determined. Click the button to try setting the mode or check the connection to the camera.",
            Message::ChangeSleeping(true),
        ),
    };

    container(tooltip(
        button(row![icon, text_element_text].spacing(5)).on_press(message),
        tooltip_content(container(tooltip_text)),
        if window_mode == WindowMode::Widget {
            Position::Bottom
        } else {
            Position::Right
        },
    ))
}
