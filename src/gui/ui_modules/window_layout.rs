// SPDX-License-Identifier: EUPL-1.2

use crate::styles::button_non_styled::button_non_styled;
use crate::styles::general_area_style::general_area_style;
use crate::ui_modules::button_sleep_wake::button_sleep_wake;
use crate::ui_modules::button_window_mode_change::button_window_mode_change;
use crate::ui_modules::current_stats::current_stats;
use crate::ui_modules::debug_area::debug_area;
use crate::ui_modules::settings_area::settings_area;
use crate::{MainPanel, Message, WindowMode};
use iced::alignment::Vertical;
use iced::widget::{
    Column, Container, Space, button, column, container, horizontal_rule, horizontal_space, image,
    row, text,
};
use iced::{Alignment, FillPortion, Length};
use rust_i18n::t;
use tiny4linux::SleepMode;
use tiny4linux_assets::handle_t4l_asset;

pub fn window_layout(app: &MainPanel) -> Container<'static, Message> {
    match app.window_mode {
        WindowMode::Dashboard => container(row![
            dashboard_general_area(app).width(Length::FillPortion(1)),
            dashboard_settings_area(app).width(Length::FillPortion(3))
        ])
        .width(Length::Fill)
        .height(Length::Fill),
        WindowMode::Widget => container(column![
            widget_head_area(app).height(Length::FillPortion(1)),
            widget_body_area(app).height(Length::FillPortion(9))
        ]),
        WindowMode::Video => container(row![
            dashboard_general_area(app).width(Length::FillPortion(1)),
            container(
                match &app.current_frame {
                    Some(frame) => container(image(frame.clone()).width(Length::Fill).height(Length::Fill)),
                    None => container(Space::new(0, 0)),
                }
            )
            .width(Length::FillPortion(1))
            .height(Length::Fill)
        ]),
        WindowMode::Invalid => container(Space::new(0, 0)),
    }
}

fn dashboard_general_area(app: &MainPanel) -> Container<'static, Message> {
    container(
        column![
            row![
                text("Tiny4Linux")
                    .size(18)
                    .height(100)
                    .align_y(Vertical::Center),
                image(handle_t4l_asset(
                    "generated/png/title-icon/v2.0-soft-shadow.png"
                ))
                .height(60)
            ]
            .align_y(Alignment::Center)
            .spacing(15),
            text(match app.awake {
                SleepMode::Awake => t!("gui.sleep.is_awake"),
                SleepMode::Sleep => t!("gui.sleep.is_sleeping"),
                SleepMode::Unknown => t!("gui.sleep.unknown"),
            }),
            button(image(if app.awake == SleepMode::Awake {
                handle_t4l_asset("generated/png/icons/inverted-camera.png")
            } else {
                handle_t4l_asset("generated/png/icons/inverted-camera-asleep.png")
            }))
            .on_press(if app.awake == SleepMode::Sleep {
                Message::ChangeSleeping(false)
            } else {
                Message::ChangeSleeping(true)
            })
            .height(100)
            .style(button_non_styled),
            button_sleep_wake(app.awake, WindowMode::Dashboard),
            horizontal_rule(8),
            row![
                horizontal_space().width(FillPortion(1)),
                current_stats(app).width(FillPortion(8)),
                horizontal_space().width(FillPortion(1))
            ]
        ]
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .spacing(10)
        .padding(5),
    )
    .height(Length::Fill)
    .style(|_| general_area_style())
}

fn dashboard_settings_area(app: &MainPanel) -> Column<'static, Message> {
    column![
        row![
            Space::with_width(Length::Fill),
            button_window_mode_change(WindowMode::Dashboard)
        ],
        settings_area(app),
        debug_area(app)
    ]
}

fn widget_head_area(app: &MainPanel) -> Container<'static, Message> {
    container(
        row![
            button_window_mode_change(WindowMode::Widget),
            row![
                image(if app.awake == SleepMode::Awake {
                    handle_t4l_asset("generated/png/icons/inverted-camera.png")
                } else {
                    handle_t4l_asset("generated/png/icons/inverted-camera-asleep.png")
                })
                .height(30),
                button_sleep_wake(app.awake, WindowMode::Widget)
            ]
            .spacing(10)
        ]
        .width(Length::Fill)
        .spacing(30),
    )
    .padding(10)
    .align_y(Vertical::Center)
    .style(|_| general_area_style())
}

fn widget_body_area(app: &MainPanel) -> Column<'static, Message> {
    column![settings_area(app)]
}
