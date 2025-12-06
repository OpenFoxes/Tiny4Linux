// SPDX-License-Identifier: EUPL-1.2

use crate::styles::tooltip_style::tooltip_content;
use crate::{Message, WindowMode};
use iced::widget::tooltip::Position;
use iced::widget::{Container, button, container, row, text, tooltip};
use iced_font_awesome::fa_icon_solid;
use rust_i18n::t;
use tiny4linux::SleepMode;

pub fn button_sleep_wake(
    sleep_mode: SleepMode,
    window_mode: WindowMode,
) -> Container<'static, Message> {
    let (text_element_text, icon, tooltip_text, message) = match sleep_mode {
        SleepMode::Awake => (
            t!("gui.buttons.sleep.set_sleep"),
            fa_icon_solid("moon"),
            t!("gui.tooltips.sleep.request_to_sleep"),
            Message::ChangeSleeping(true),
        ),
        SleepMode::Sleep => (
            t!("gui.buttons.sleep.wake_up"),
            fa_icon_solid("sun"),
            t!("gui.tooltips.sleep.request_to_wake"),
            Message::ChangeSleeping(false),
        ),
        SleepMode::Unknown => (
            t!("gui.buttons.sleep.unknown"),
            fa_icon_solid("question"),
            t!("gui.tooltips.sleep.request_to_sleep"),
            Message::ChangeSleeping(true),
        ),
    };

    container(tooltip(
        button(row![icon, text(text_element_text)].spacing(5)).on_press(message),
        tooltip_content(container(text(tooltip_text))),
        if window_mode == WindowMode::Widget {
            Position::Bottom
        } else {
            Position::Right
        },
    ))
}
