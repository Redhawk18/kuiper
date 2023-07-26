use iced::widget::{button, text, text_input};
use iced::{application, color, Background};
use iced_aw::menu;
use iced_aw::style::tab_bar; //FIXME https://github.com/iced-rs/iced_aw/issues/151

mod colors;

use colors::Colors;

pub type Renderer = iced::Renderer<Theme>;
pub type Element<'msg, Message> = iced::Element<'msg, Message, Renderer>;

#[derive(Clone)]
pub struct Theme {
    pub colors: Colors,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            colors: Colors::default(),
        }
    }
}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.colors.background,
            text_color: self.colors.text,
        }
    }
}
#[derive(Default)]
pub enum Button {
    #[default]
    Primary,
}

impl button::StyleSheet for Theme {
    type Style = Button;
    #[allow(unused_variables)]
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub enum Menu {
    #[default]
    Primary,
}
impl menu::StyleSheet for Theme {
    type Style = Menu;
    #[allow(unused_variables)]
    fn appearance(&self, style: &Self::Style) -> menu::Appearance {
        Default::default()
    }
}

#[derive(Clone, Copy, Default)]
pub enum TabBar {
    #[default]
    Primary,
}

impl tab_bar::StyleSheet for Theme {
    type Style = TabBar;
    #[allow(unused_variables)]
    fn active(&self, style: Self::Style, is_active: bool) -> tab_bar::Appearance {
        match style {
            TabBar::Primary => tab_bar::Appearance {
                background: Some(Background::Color(self.colors.background)),
                border_color: Some(self.colors.accent),
                border_width: 4.0,
                icon_color: self.colors.accent,
                tab_label_background: Background::Color(self.colors.accent),
                tab_label_border_color: self.colors.accent,
                tab_label_border_width: 1.0,
                text_color: self.colors.accent
            }
        }
    }
    #[allow(unused_variables)]
    fn hovered(&self, style: Self::Style, is_active: bool) -> tab_bar::Appearance {
        Default::default()
    }
}

#[derive(Clone, Copy, Default)]
pub enum Text {
    #[default]
    Primary,
}

impl text::StyleSheet for Theme {
    type Style = Text;
    #[allow(unused_variables)]
    fn appearance(&self, style: Self::Style) -> text::Appearance {
        Default::default()
    }
}

#[derive(Default)]
pub enum TextInput {
    #[default]
    Primary,
}

impl text_input::StyleSheet for Theme {
    type Style = TextInput;
    #[allow(unused_variables)]
    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        match style {
            TextInput::Primary => text_input::Appearance {
                background: Background::Color(self.colors.background),
                border_radius: 4.0.into(),
                border_width: 1.0,
                border_color: self.colors.accent,
                // XXX Not currently displayed in application.
                icon_color: self.colors.accent,
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }

    #[allow(unused_variables)]
    fn disabled_color(&self, style: &Self::Style) -> iced::Color {
        Default::default()
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }

    #[allow(unused_variables)]
    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        Default::default()
    }

    fn selection_color(&self, style: &Self::Style) -> iced::Color {
        match style {
            TextInput::Primary => self.colors.accent,
        }
    }

    fn value_color(&self, style: &Self::Style) -> iced::Color {
        match style {
            TextInput::Primary => self.colors.text,
        }
    }
}
