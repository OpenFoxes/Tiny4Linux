use crate::Message;
use crate::styles::tooltip_style::tooltip_content;
use iced::alignment::Vertical;
use iced::widget::tooltip::Position;
use iced::widget::{Container, button, container, row, text, tooltip};
use iced_font_awesome::fa_icon_solid;

pub fn button_hdr(current_mode: bool) -> Container<'static, Message> {
    container(tooltip(
        button(
            row![
                fa_icon_solid(if current_mode { "palette" } else { "power-off" }),
                text(if current_mode { "HDR on" } else { "HDR off" })
            ]
            .align_y(Vertical::Center)
            .spacing(5),
        )
        .on_press(Message::ChangeHDR(!current_mode)),
        tooltip_content(container(if current_mode {
            "Turns off HDR on click"
        } else {
            "Turns on HDR on click"
        })),
        Position::Bottom,
    ))
}
