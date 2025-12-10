// SPDX-License-Identifier: EUPL-1.2

use crate::{MainPanel, Message};
use iced::alignment::Horizontal;
use iced::widget::{Container, column, container, horizontal_rule, row, text};
use iced_font_awesome::fa_icon_solid;
use rust_i18n::t;
use tiny4linux::{AIMode, SleepMode, TrackingSpeed};

const TEXT_INDENT: &str = "   ";
const TEXT_INDENT_LONG: &str = "      ";

pub fn current_stats(app: &MainPanel) -> Container<'static, Message> {
    container(
        column![
            text(format!("{}:", t!("shared.info.camera_status"))),
            column![
                row![
                    text(TEXT_INDENT),
                    fa_icon_solid("moon"),
                    text(format!("{}:", t!("shared.info.sleep_mode")))
                ]
                .spacing(10),
                row![
                    text(TEXT_INDENT_LONG),
                    match app.awake {
                        SleepMode::Awake =>
                            row![fa_icon_solid("eye"), text(t!("display.sleep_mode.awake"))]
                                .spacing(10),
                        SleepMode::Sleep => row![
                            fa_icon_solid("eye-slash"),
                            text(t!("display.sleep_mode.sleep"))
                        ]
                        .spacing(10),
                        SleepMode::Unknown => row![
                            fa_icon_solid("question-circle"),
                            text(t!("display.sleep_mode.unknown"))
                        ]
                        .spacing(10),
                    }
                ]
                .spacing(10),
                horizontal_rule(1),
                row![
                    text(TEXT_INDENT),
                    fa_icon_solid("robot"),
                    text(format!("{}:", t!("shared.info.ai_mode"))),
                ]
                .spacing(10),
                row![
                    text(TEXT_INDENT_LONG),
                    match app.tracking {
                        AIMode::NoTracking => {
                            row![fa_icon_solid("tape"), text(t!("display.ai_mode.static"))]
                                .spacing(10)
                        }
                        AIMode::NormalTracking => {
                            row![
                                fa_icon_solid("user"),
                                text(t!("display.ai_mode.normal_short"))
                            ]
                            .spacing(10)
                        }
                        AIMode::UpperBody => {
                            row![
                                fa_icon_solid("user-plus"),
                                text(t!("display.ai_mode.upper_body"))
                            ]
                            .spacing(10)
                        }
                        AIMode::CloseUp => {
                            row![
                                fa_icon_solid("face-smile"),
                                text(t!("display.ai_mode.close_up"))
                            ]
                            .spacing(10)
                        }
                        AIMode::Headless => {
                            row![
                                fa_icon_solid("circle-xmark"),
                                text(t!("display.ai_mode.headless"))
                            ]
                            .spacing(10)
                        }
                        AIMode::LowerBody => {
                            row![
                                fa_icon_solid("down-long"),
                                text(t!("display.ai_mode.lower_body"))
                            ]
                            .spacing(10)
                        }
                        AIMode::DeskMode => {
                            row![fa_icon_solid("stapler"), text(t!("display.ai_mode.desk"))]
                                .spacing(10)
                        }
                        AIMode::Whiteboard => {
                            row![
                                fa_icon_solid("chalkboard"),
                                text(t!("display.ai_mode.whiteboard"))
                            ]
                            .spacing(10)
                        }
                        AIMode::Hand => {
                            row![fa_icon_solid("hand"), text(t!("display.ai_mode.hand"))]
                                .spacing(10)
                        }
                        AIMode::Group => {
                            row![
                                fa_icon_solid("users-viewfinder"),
                                text(t!("display.ai_mode.group"))
                            ]
                            .spacing(10)
                        }
                        AIMode::Unknown => {
                            row![
                                fa_icon_solid("question-circle"),
                                text(t!("display.ai_mode.unknown"))
                            ]
                            .spacing(10)
                        }
                    }
                ]
                .spacing(10),
                horizontal_rule(1),
                row![
                    text(TEXT_INDENT),
                    fa_icon_solid("gauge"),
                    text(format!("{}:", t!("shared.info.tracking_speed"))),
                ]
                .spacing(10),
                row![
                    text(TEXT_INDENT_LONG),
                    match app.tracking_speed {
                        TrackingSpeed::Standard => row![
                            fa_icon_solid("gauge-simple"),
                            text(t!("display.tracking_speed.standard"))
                        ]
                        .spacing(10),
                        TrackingSpeed::Sport => row![
                            fa_icon_solid("gauge-simple-high"),
                            text(t!("display.tracking_speed.sport"))
                        ]
                        .spacing(10),
                    }
                ]
                .spacing(10),
                horizontal_rule(1),
                row![
                    text(TEXT_INDENT),
                    fa_icon_solid("palette"),
                    text(format!("{}:", t!("shared.info.hdr"))),
                ]
                .spacing(10),
                row![
                    text(TEXT_INDENT_LONG),
                    if app.hdr_on {
                        row![fa_icon_solid("toggle-on"), text(t!("display.states.on"))].spacing(10)
                    } else {
                        row![fa_icon_solid("toggle-off"), text(t!("display.states.off"))]
                            .spacing(10)
                    }
                ]
                .spacing(10),
                horizontal_rule(1),
                row![
                    text(TEXT_INDENT),
                    fa_icon_solid("code-commit"),
                    text(format!("{}:", t!("shared.info.t4l_version"))),
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
