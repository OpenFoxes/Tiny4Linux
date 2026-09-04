// SPDX-License-Identifier: EUPL-1.2

use crate::styles::colors::COLOR_BACKGROUND_SECONDARY_DARK;
use iced::widget::button::{Status, Style};
use iced::{Background, Color, Theme};

/// Style for a control the connected camera does not have (#72).
///
/// iced dims a disabled button only slightly, which on the dark theme is easy to
/// miss. This fades it much further instead. It deliberately has no outline: an
/// outlined control reads as interactive, and the active buttons next to it are
/// solid fills, so a border here would give the unavailable control the stronger
/// visual cue of the two.
pub fn button_unsupported(_: &Theme, _: Status) -> Style {
    Style {
        background: Some(Background::Color(Color {
            a: 0.30,
            ..COLOR_BACKGROUND_SECONDARY_DARK
        })),
        text_color: Color {
            a: 0.28,
            ..Color::WHITE
        },
        ..Default::default()
    }
}
