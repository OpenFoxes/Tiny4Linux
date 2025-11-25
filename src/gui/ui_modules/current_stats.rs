// SPDX-License-Identifier: EUPL-1.2

use crate::{MainPanel, Message};
use iced::alignment::Horizontal;
use iced::widget::{Container, column, container, horizontal_rule, row, text};
use iced_font_awesome::fa_icon_solid;
use tiny4linux::{AIMode, SleepMode, TrackingSpeed};

const TEXT_INDENT: &str = "   ";
const TEXT_INDENT_LONG: &str = "      ";

pub fn current_stats(app: &MainPanel) -> Container<'static, Message> {
    container(
        column![
            text("Camera status:"),
            column![
                row![
                    text(TEXT_INDENT),
                    fa_icon_solid("moon"),
                    text("Sleep Mode:")
                ]
                .spacing(10),
                row![
                    text(TEXT_INDENT_LONG),
                    match app.awake {
                        SleepMode::Awake => row![fa_icon_solid("eye"), text("Awake")].spacing(10),
                        SleepMode::Sleep =>
                            row![fa_icon_solid("eye-slash"), text("Sleeping")].spacing(10),
                        SleepMode::Unknown =>
                            row![fa_icon_solid("question-circle"), text("Unknown")].spacing(10),
                    }
                ]
                .spacing(10),
                horizontal_rule(1),
                row![text(TEXT_INDENT), fa_icon_solid("robot"), text("AI Mode:"),].spacing(10),
                row![
                    text(TEXT_INDENT_LONG),
                    match app.tracking {
                        AIMode::NoTracking => {
                            row![fa_icon_solid("tape"), text("Static")].spacing(10)
                        }
                        AIMode::NormalTracking => {
                            row![fa_icon_solid("user"), text("Normal")].spacing(10)
                        }
                        AIMode::UpperBody => {
                            row![fa_icon_solid("user-plus"), text("Upper Body")].spacing(10)
                        }
                        AIMode::CloseUp => {
                            row![fa_icon_solid("face-smile"), text("Close Up")].spacing(10)
                        }
                        AIMode::Headless => {
                            row![fa_icon_solid("circle-xmark"), text("Headless Body")].spacing(10)
                        }
                        AIMode::LowerBody => {
                            row![fa_icon_solid("down-long"), text("Lower Body")].spacing(10)
                        }
                        AIMode::DeskMode => {
                            row![fa_icon_solid("stapler"), text("Desk")].spacing(10)
                        }
                        AIMode::Whiteboard => {
                            row![fa_icon_solid("chalkboard"), text("Whiteboard")].spacing(10)
                        }
                        AIMode::Hand => {
                            row![fa_icon_solid("hand"), text("Hand Tracking")].spacing(10)
                        }
                        AIMode::Group => {
                            row![fa_icon_solid("users-viewfinder"), text("Group")].spacing(10)
                        }
                        AIMode::Unknown => {
                            row![fa_icon_solid("question-circle"), text("Unknown")].spacing(10)
                        }
                    }
                ]
                .spacing(10),
                horizontal_rule(1),
                row![
                    text(TEXT_INDENT),
                    fa_icon_solid("gauge"),
                    text("Tracking Speed:"),
                ]
                .spacing(10),
                row![
                    text(TEXT_INDENT_LONG),
                    match app.tracking_speed {
                        TrackingSpeed::Standard =>
                            row![fa_icon_solid("gauge-simple"), text("Standard")].spacing(10),
                        TrackingSpeed::Sport =>
                            row![fa_icon_solid("gauge-simple-high"), text("Sport")].spacing(10),
                    }
                ]
                .spacing(10),
                horizontal_rule(1),
                row![text(TEXT_INDENT), fa_icon_solid("palette"), text("HDR:"),].spacing(10),
                row![
                    text(TEXT_INDENT_LONG),
                    if app.hdr_on {
                        row![fa_icon_solid("toggle-on"), text("On")].spacing(10)
                    } else {
                        row![fa_icon_solid("toggle-off"), text("Off")].spacing(10)
                    }
                ]
                .spacing(10),
                horizontal_rule(1),
                row![
                    text(TEXT_INDENT),
                    fa_icon_solid("code-commit"),
                    text("T4L-Version:"),
                ]
                .spacing(10),
                row![text(TEXT_INDENT_LONG), text(env!("CARGO_PKG_VERSION"))].spacing(10)
            ]
            .spacing(10)
        ]
        .spacing(15)
        .align_x(Horizontal::Left),
    )
}
