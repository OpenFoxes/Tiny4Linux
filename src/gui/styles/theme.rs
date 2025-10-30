use crate::styles::colors::{COLOR_BACKGROUND_DARK, COLOR_PRIMARY_OBSBOT};
use iced::Theme;
use iced::theme::Palette;
use iced::theme::palette::Extended;

pub fn obsbot_theme() -> Theme {
    let palette = Palette {
        primary: COLOR_PRIMARY_OBSBOT,
        background: COLOR_BACKGROUND_DARK,
        success: Default::default(),
        text: Default::default(),
        danger: Default::default(),
    };

    Extended::generate(palette);

    Theme::custom("Tiny4Linux Default Theme".to_string(), palette)
}
