use iced::widget::button;
use iced::{application, Color, color, Theme};

impl application::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> Appearance {
        match style {
            Button::Primary => button::Appearance {
                background: color!(0x28, 0x28, 0x28).into(),
                ..Default::default()
            }
        }
    }
}
