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
use tiny4linux::{AIMode, CameraModel, ExposureMode, TrackingSpeed};

pub fn settings_area(app: &MainPanel) -> Container<'static, Message> {
    container(
        column![
            presets(),
            horizontal_rule(8),
            tracking_modes(
                app.window_mode == WindowMode::Widget,
                app.tracking,
                app.camera.as_ref().map(|camera| camera.model()),
            ),
            tracking_speed(
                app.tracking_speed,
                app.camera.as_ref().map(|camera| camera.model())
            ),
            horizontal_rule(8),
            row![
                hdr(app.hdr_on),
                exposure_mode(app.camera.as_ref().map(|camera| camera.model()))
            ]
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

/// `model` is `None` while no camera is connected, in which case every button stays
/// enabled, exactly as before.
fn tracking_modes(
    reduced: bool,
    current_mode: AIMode,
    model: Option<CameraModel>,
) -> Container<'static, Message> {
    let supported = |mode: AIMode| model.is_none_or(|model| model.supports_ai_mode(mode));
    container(
        column![
            text(format!("{}:", t!("shared.info.tracking"))),
            if reduced {
                column![
                    row![
                        button_tracking_mode(
                            AIMode::NoTracking,
                            current_mode,
                            supported(AIMode::NoTracking)
                        ),
                        button_tracking_mode(
                            AIMode::NormalTracking,
                            current_mode,
                            supported(AIMode::NormalTracking)
                        ),
                    ]
                    .spacing(10),
                    row![
                        button_tracking_mode(AIMode::Hand, current_mode, supported(AIMode::Hand)),
                        button_tracking_mode(
                            AIMode::Whiteboard,
                            current_mode,
                            supported(AIMode::Whiteboard)
                        ),
                        button_tracking_mode(AIMode::Group, current_mode, supported(AIMode::Group)),
                    ]
                    .spacing(10)
                ]
                .spacing(10)
                .align_x(Horizontal::Center)
            } else {
                column![
                    row![
                        button_tracking_mode(
                            AIMode::NoTracking,
                            current_mode,
                            supported(AIMode::NoTracking)
                        ),
                        button_tracking_mode(
                            AIMode::NormalTracking,
                            current_mode,
                            supported(AIMode::NormalTracking)
                        ),
                    ]
                    .spacing(10),
                    row![
                        button_tracking_mode(
                            AIMode::CloseUp,
                            current_mode,
                            supported(AIMode::CloseUp)
                        ),
                        button_tracking_mode(
                            AIMode::UpperBody,
                            current_mode,
                            supported(AIMode::UpperBody)
                        ),
                        button_tracking_mode(
                            AIMode::Headless,
                            current_mode,
                            supported(AIMode::Headless)
                        ),
                        button_tracking_mode(
                            AIMode::LowerBody,
                            current_mode,
                            supported(AIMode::LowerBody)
                        ),
                    ]
                    .spacing(10),
                    row![
                        button_tracking_mode(
                            AIMode::DeskMode,
                            current_mode,
                            supported(AIMode::DeskMode)
                        ),
                        button_tracking_mode(
                            AIMode::Whiteboard,
                            current_mode,
                            supported(AIMode::Whiteboard)
                        ),
                        button_tracking_mode(AIMode::Hand, current_mode, supported(AIMode::Hand)),
                        button_tracking_mode(AIMode::Group, current_mode, supported(AIMode::Group)),
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

fn tracking_speed(
    current_speed: TrackingSpeed,
    model: Option<CameraModel>,
) -> Container<'static, Message> {
    let supported = model.is_none_or(|model| model.supports_tracking_speed());
    container(
        column![
            text(format!("{}:", t!("shared.info.tracking_speed"))),
            column![
                row![
                    button_tracking_speed(TrackingSpeed::Standard, current_speed, supported),
                    button_tracking_speed(TrackingSpeed::Sport, current_speed, supported),
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

fn exposure_mode(model: Option<CameraModel>) -> Container<'static, Message> {
    container(
        column![
            text(format!("{}:", t!("shared.info.exposure"))),
            button_exposure_mode(
                ExposureMode::Manual,
                model.is_none_or(|model| model.supports_manual_exposure()),
            ),
            button_exposure_mode(ExposureMode::Global, true),
            button_exposure_mode(ExposureMode::Face, true),
        ]
        .align_x(Horizontal::Center)
        .width(Length::Fill)
        .spacing(5),
    )
}
