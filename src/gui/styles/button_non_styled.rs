use iced::Theme;
use iced::widget::button::{Status, Style};

pub fn button_non_styled(_: &Theme, _: Status) -> Style {
    Style {
        background: None,
        ..Default::default()
    }
}
