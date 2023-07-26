use iced::widget::{button, text, text_input};
use iced::{application, color, Background};
use iced_aw::menu;
use iced_aw::style::tab_bar; //FIXME https://github.com/iced-rs/iced_aw/issues/151

mod colors;

use colors::Colors;

pub type Renderer = iced::Renderer<Theme>;
pub type Element<'msg, Message> = iced::Element<'msg, Message, Renderer>;

#[derive(Debug, Clone, Copy, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl application::StyleSheet for Theme {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: color!(255, 255, 255),
            text_color: color!(0, 0, 0),
        }
    }
}

impl button::StyleSheet for Theme {
    type Style = Theme;
    #[allow(unused_variables)]
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..Default::default()
        }
    }
}

impl menu::StyleSheet for Theme {
    type Style = Theme;
    #[allow(unused_variables)]
    fn appearance(&self, style: &Self::Style) -> menu::Appearance {
        Default::default()
    }
}

impl tab_bar::StyleSheet for Theme {
    type Style = Theme;
    #[allow(unused_variables)]
    fn active(&self, style: Self::Style, is_active: bool) -> tab_bar::Appearance {
        Default::default()
    }
    #[allow(unused_variables)]
    fn hovered(&self, style: Self::Style, is_active: bool) -> tab_bar::Appearance {
        Default::default()
    }
}

impl text::StyleSheet for Theme {
    type Style = Theme;
    #[allow(unused_variables)]
    fn appearance(&self, style: Self::Style) -> text::Appearance {
        Default::default()
    }
}

impl text_input::StyleSheet for Theme {
    type Style = Theme;
    #[allow(unused_variables)]
    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(color!(255, 255, 255)),
            border_radius: 4.0.into(),
            border_width: 0.0,
            border_color: iced::Color::TRANSPARENT,
            // XXX Not currently displayed in application.
            icon_color: color!(0, 0, 0),
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }
    #[allow(unused_variables)]
    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        Default::default()
    }
    #[allow(unused_variables)]
    fn value_color(&self, style: &Self::Style) -> iced::Color {
        Default::default()
    }
    #[allow(unused_variables)]
    fn disabled_color(&self, style: &Self::Style) -> iced::Color {
        Default::default()
    }
    #[allow(unused_variables)]
    fn selection_color(&self, style: &Self::Style) -> iced::Color {
        Default::default()
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }
}
