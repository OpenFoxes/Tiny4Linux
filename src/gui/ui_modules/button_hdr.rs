// SPDX-License-Identifier: EUPL-1.2

use crate::Message;
use crate::styles::tooltip_style::tooltip_content;
use iced::alignment::Vertical;
use iced::widget::tooltip::Position;
use iced::widget::{Container, button, container, row, text, tooltip};
use iced_font_awesome::fa_icon_solid;
use rust_i18n::t;

pub fn button_hdr(current_mode: bool) -> Container<'static, Message> {
    container(tooltip(
        button(
            row![
                fa_icon_solid(if current_mode { "palette" } else { "power-off" }),
                text(if current_mode {
                    t!("shared.options.hdr.on")
                } else {
                    t!("shared.options.hdr.off")
                })
            ]
            .align_y(Vertical::Center)
            .spacing(5),
        )
        .on_press(Message::ChangeHDR(!current_mode)),
        tooltip_content(container(text(if current_mode {
            t!("gui.tooltips.hdr.turns_off")
        } else {
            t!("gui.tooltips.hdr.turns_on")
        }))),
        Position::Bottom,
    ))
}
