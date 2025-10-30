use crate::Message;
use crate::styles::colors::{COLOR_BACKGROUND_DARK, COLOR_BACKGROUND_SECONDARY_DARK};
use iced::Border;
use iced::border::radius;
use iced::widget::{Container, container};

pub fn tooltip_content(content: Container<Message>) -> Container<Message> {
    content.padding(10).style(|_| tooltip_style())
}

fn tooltip_style() -> container::Style {
    container::Style {
        background: Some(COLOR_BACKGROUND_SECONDARY_DARK.into()),
        border: Border {
            color: COLOR_BACKGROUND_DARK,
            width: 2.0,
            radius: radius(0),
        },
        ..Default::default()
    }
}
