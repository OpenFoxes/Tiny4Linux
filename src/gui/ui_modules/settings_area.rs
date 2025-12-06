// SPDX-License-Identifier: EUPL-1.2

use crate::styles::tooltip_style::tooltip_content;
use crate::ui_modules::button_exposure_mode::button_exposure_mode;
use crate::ui_modules::button_hdr::button_hdr;
use crate::ui_modules::button_tracking_mode::{button_tracking_mode, button_tracking_speed};
use crate::{MainPanel, Message, WindowMode};
use iced::Length;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::tooltip::Position;
use iced::widget::{
    Container, Row, button, column, container, horizontal_rule, horizontal_space, row, text,
    tooltip,
};
use iced_font_awesome::fa_icon_solid;
use rust_i18n::t;
use tiny4linux::{AIMode, ExposureMode, TrackingSpeed};

pub fn settings_area(app: &MainPanel) -> Container<'static, Message> {
    container(
        column![
            presets(),
            horizontal_rule(8),
            tracking_modes(app.window_mode == WindowMode::Widget, app.tracking),
            tracking_speed(app.tracking_speed),
            horizontal_rule(8),
            row![hdr(app.hdr_on), exposure_mode()]
                .spacing(10)
                .align_y(Vertical::Center),
        ]
        .spacing(20),
    )
    .padding(10)
}

fn presets() -> Row<'static, Message> {
    row![
        text(format!("{}:", t!("shared.info.presets"))),
        horizontal_space().width(Length::FillPortion(2)),
        (0..=2)
            .fold(row![], |r, n| {
                let r = r.push(tooltip(
                    button(fa_icon_solid(&(n + 1).to_string()))
                        .on_press(Message::ChangePresetPosition(n)),
                    tooltip_content(container(text(t!(
                        "gui.tooltips.preset",
                        preset_number = n + 1
                    )))),
                    Position::Bottom,
                ));
                r.push(horizontal_space().width(Length::FillPortion(1)))
            })
            .width(Length::FillPortion(6)),
        horizontal_space().width(Length::FillPortion(2))
    ]
}

fn tracking_modes(reduced: bool, current_mode: AIMode) -> Container<'static, Message> {
    container(
        column![
            text(format!("{}:", t!("shared.info.tracking"))),
            if reduced {
                column![
                    row![
                        button_tracking_mode(AIMode::NoTracking, current_mode),
                        button_tracking_mode(AIMode::NormalTracking, current_mode),
                    ]
                    .spacing(10),
                    row![
                        button_tracking_mode(AIMode::Hand, current_mode),
                        button_tracking_mode(AIMode::Whiteboard, current_mode),
                        button_tracking_mode(AIMode::Group, current_mode),
                    ]
                    .spacing(10)
                ]
                .spacing(10)
                .align_x(Horizontal::Center)
            } else {
                column![
                    row![
                        button_tracking_mode(AIMode::NoTracking, current_mode),
                        button_tracking_mode(AIMode::NormalTracking, current_mode),
                    ]
                    .spacing(10),
                    row![
                        button_tracking_mode(AIMode::CloseUp, current_mode),
                        button_tracking_mode(AIMode::UpperBody, current_mode),
                        button_tracking_mode(AIMode::Headless, current_mode),
                        button_tracking_mode(AIMode::LowerBody, current_mode),
                    ]
                    .spacing(10),
                    row![
                        button_tracking_mode(AIMode::DeskMode, current_mode),
                        button_tracking_mode(AIMode::Whiteboard, current_mode),
                        button_tracking_mode(AIMode::Hand, current_mode),
                        button_tracking_mode(AIMode::Group, current_mode),
                    ]
                    .spacing(10)
                ]
                .spacing(10)
                .width(Length::Fill)
                .align_x(Horizontal::Center)
            }
        ]
        .width(Length::Fill)
        .spacing(10),
    )
}

fn tracking_speed(current_speed: TrackingSpeed) -> Container<'static, Message> {
    container(
        column![
            text(format!("{}:", t!("shared.info.tracking_speed"))),
            column![
                row![
                    button_tracking_speed(TrackingSpeed::Standard, current_speed),
                    button_tracking_speed(TrackingSpeed::Sport, current_speed),
                ]
                .spacing(10),
            ]
            .align_x(Horizontal::Center)
            .width(Length::Fill)
        ]
        .spacing(10)
        .width(Length::Fill),
    )
}

fn hdr(current_mode: bool) -> Container<'static, Message> {
    container(
        column![
            text(format!("{}:", t!("shared.info.hdr"))),
            button_hdr(current_mode)
        ]
        .spacing(5)
        .align_x(Horizontal::Center)
        .width(Length::Fill),
    )
}

fn exposure_mode() -> Container<'static, Message> {
    container(
        column![
            text(format!("{}:", t!("shared.info.exposure"))),
            button_exposure_mode(ExposureMode::Manual),
            button_exposure_mode(ExposureMode::Global),
            button_exposure_mode(ExposureMode::Face),
        ]
        .align_x(Horizontal::Center)
        .width(Length::Fill)
        .spacing(5),
    )
}
