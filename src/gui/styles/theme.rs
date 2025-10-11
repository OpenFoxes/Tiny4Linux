use iced::theme::Palette;
use iced::theme::palette::Extended;
use iced::{Color, Theme};

pub fn obsbot_theme() -> Theme {
    let palette = Palette {
        primary: Color::from_rgb8(0xe6, 0x00, 0x33),
        background: Color::from_rgb8(0x19, 0x1a, 0x1b),
        success: Default::default(),
        text: Default::default(),
        danger: Default::default(),
    };

    Extended::generate(palette);

    Theme::custom("Tiny4Linux Default Theme".to_string(), palette)
}
