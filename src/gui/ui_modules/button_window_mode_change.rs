// SPDX-License-Identifier: EUPL-1.2

use crate::styles::tooltip_style::tooltip_content;
use crate::{Message, WindowMode};
use iced::widget::tooltip::Position;
use iced::widget::{Container, Space, button, container, tooltip};
use iced_font_awesome::fa_icon_solid;

pub fn button_window_mode_change(window_mode: WindowMode) -> Container<'static, Message> {
    match window_mode {
        WindowMode::Dashboard => container(tooltip(
            button(fa_icon_solid("down-left-and-up-right-to-center"))
                .on_press(Message::RequestWindowModeChange(WindowMode::Widget)),
            tooltip_content(container("Switch to Widget-Mode")),
            Position::Bottom,
        )),
        WindowMode::Widget => container(tooltip(
            button(fa_icon_solid("up-right-and-down-left-from-center"))
                .on_press(Message::RequestWindowModeChange(WindowMode::Dashboard)),
            tooltip_content(container("Switch to Dashboard-Mode")),
            Position::Bottom,
        )),
        WindowMode::Invalid => container(Space::new(0, 0)),
    }
}
