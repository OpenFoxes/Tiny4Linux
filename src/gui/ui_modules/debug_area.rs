// SPDX-License-Identifier: EUPL-1.2

use crate::Message::{HexDump, HexDump02};
use crate::{MainPanel, Message};
use iced::Length;
use iced::alignment::Horizontal;
use iced::widget::button::{primary, secondary};
use iced::widget::{Container, button, column, container, horizontal_space, row, text, text_input};
use iced_font_awesome::fa_icon_solid;

const DEBUG_BUTTON_WIDTH: f32 = 100.0;
const DEBUG_INPUT_WIDTH: f32 = 250.0;

pub fn debug_area(app: &MainPanel) -> Container<'static, Message> {
    let debugging_active = app.debugging_on;

    container(
        column![
            button(if debugging_active {
                row![fa_icon_solid("bug"), text("Turn off debugging")].spacing(5)
            } else {
                row![fa_icon_solid("bug-slash"), text("Turn on debugging")].spacing(5)
            })
            .on_press(Message::ChangeDebugging(!debugging_active))
            .style(if debugging_active { primary } else { secondary }),
            if debugging_active {
                column![
                    row![
                        row![fa_icon_solid("paper-plane"), text("Send")].spacing(5),
                        horizontal_space(),
                        column![
                            row![
                                text_input("0x06 hex string", &app.text_input)
                                    .on_input(Message::TextInput)
                                    .on_submit(Message::SendCommand)
                                    .width(DEBUG_INPUT_WIDTH),
                                button("Send 0x06")
                                    .on_press(Message::SendCommand)
                                    .width(DEBUG_BUTTON_WIDTH),
                                button("Clear")
                                    .on_press(Message::TextInput("".parse().unwrap()))
                                    .width(DEBUG_BUTTON_WIDTH)
                            ]
                            .spacing(15),
                            row![
                                text_input("0x02 hex string", &app.text_input_02)
                                    .on_input(Message::TextInput02)
                                    .on_submit(Message::SendCommand02)
                                    .width(DEBUG_INPUT_WIDTH),
                                button("Send 0x02")
                                    .on_press(Message::SendCommand02)
                                    .width(DEBUG_BUTTON_WIDTH),
                                button("Clear")
                                    .on_press(Message::TextInput02("".parse().unwrap()))
                                    .width(DEBUG_BUTTON_WIDTH)
                            ]
                            .spacing(15),
                        ]
                        .spacing(15),
                    ],
                    row![
                        row![fa_icon_solid("satellite-dish"), text("Get & Dump")].spacing(5),
                        horizontal_space(),
                        button("0x06 hex")
                            .width(DEBUG_BUTTON_WIDTH)
                            .on_press(HexDump),
                        button("0x02 hex")
                            .width(DEBUG_BUTTON_WIDTH)
                            .on_press(HexDump02),
                    ]
                    .spacing(15)
                ]
                .spacing(10)
            } else {
                column![]
            }
        ]
        .spacing(10)
        .align_x(Horizontal::Center)
        .width(Length::Fill),
    )
}
