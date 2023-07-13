use iced::{application, color};

pub mod mywidget {
    use super::Theme;
    pub type Renderer = iced::Renderer<Theme>;
    pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
    pub type Button<'a, Message> = iced::widget::Button<'a, Message, Renderer>;
    pub type Text<'a> = iced::widget::Text<'a, Renderer>;
    pub type Row<'a, Message> = iced::widget::Row<'a, Message, Renderer>;
    pub type Column<'a, Message> = iced::widget::Column<'a, Message, Renderer>;
    pub type Scrollable<'a, Message> = iced::widget::Scrollable<'a, Message, Renderer>;
}

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
