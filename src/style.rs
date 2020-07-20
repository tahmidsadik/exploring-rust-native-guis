use iced::{button, container, Background, Color};

pub enum Button {
    BGColor(Color),
}

pub struct Container;

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        match self {
            Button::BGColor(col) => button::Style {
                background: Some(Background::Color(*col)),
                ..button::Style::default()
            },
        }
    }
}

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
            ..container::Style::default()
        }
    }
}
