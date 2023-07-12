use iced::{application, color};

#[derive(Debug, Clone, Copy, Default)]
pub enum Theme {
  #[default]
  Dark,
  Light,
}

impl application::StyleSheet for Theme {
    type Style = Theme; // ModernTheme in this case is an enum

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: color!(0, 0, 0),
            text_color: color!(150, 150, 150),
        }
    }
}
