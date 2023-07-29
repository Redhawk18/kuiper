use iced::widget::{button, text, text_input};
use iced::{application, Background};
use iced_aw::menu;
use iced_aw::style::tab_bar; //FIXME https://github.com/iced-rs/iced_aw/issues/151

mod colors;

use colors::Colors;

pub type Renderer = iced::Renderer<Theme>;
pub type Element<'msg, Message> = iced::Element<'msg, Message, Renderer>;

#[derive(Clone, Default)]
pub struct Theme {
    pub colors: Colors,
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

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Primary => button::Appearance {
                background: None,
                border_color: self.colors.accent,
                border_radius: 0.0,
                border_width: 0.0,
                shadow_offset: iced::Vector::default(),
                text_color: self.colors.text,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Primary => button::Appearance {
                background: Some(Background::Color(self.colors.accent)),
                border_color: self.colors.accent,
                border_radius: 0.0,
                border_width: 0.0,
                shadow_offset: iced::Vector::default(),
                text_color: self.colors.text,
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Primary => button::Appearance {
                background: Some(Background::Color(self.colors.accent)),
                border_color: self.colors.accent,
                border_radius: 0.0,
                border_width: 0.0,
                shadow_offset: iced::Vector::default(),
                text_color: self.colors.text,
            },
        }
    }

    //fn disabled(&self, style: &Self::Style) -> button::Appearance {} //todo
}

#[derive(Default)]
pub enum Menu {
    #[default]
    Primary,
}
impl menu::StyleSheet for Theme {
    type Style = Menu;

    fn appearance(&self, style: &Self::Style) -> menu::Appearance {
        match style {
            Menu::Primary => menu::Appearance {
                background: self.colors.background,
                border_width: 2.0,
                border_radius: [4.0; 4],
                border_color: self.colors.primary,
                background_expand: [0; 4],
                path: self.colors.accent,
            },
        }
    }
}

#[derive(Clone, Copy, Default)]
pub enum TabBar {
    #[default]
    Primary,
}

impl tab_bar::StyleSheet for Theme {
    //TODO add is_active
    type Style = TabBar;

    fn active(&self, style: Self::Style, is_active: bool) -> tab_bar::Appearance {
        match style {
            TabBar::Primary => tab_bar::Appearance {
                background: Some(Background::Color(self.colors.background)),
                border_color: Some(self.colors.primary),
                border_width: 4.0,
                icon_color: self.colors.accent,
                tab_label_background: Background::Color(if is_active {
                    self.colors.primary
                } else {
                    self.colors.secondary
                }),
                tab_label_border_color: self.colors.primary,
                tab_label_border_width: 1.0,
                text_color: self.colors.text,
            },
        }
    }

    fn hovered(&self, style: Self::Style, _is_active: bool) -> tab_bar::Appearance {
        match style {
            TabBar::Primary => tab_bar::Appearance {
                background: Some(Background::Color(self.colors.background)),
                border_color: Some(self.colors.accent),
                border_width: 4.0,
                icon_color: self.colors.accent,
                tab_label_background: Background::Color(self.colors.primary),
                tab_label_border_color: self.colors.accent,
                tab_label_border_width: 1.0,
                text_color: self.colors.text,
            },
        }
    }
}

#[derive(Clone, Copy, Default)]
pub enum Text {
    #[default]
    Primary,
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            Text::Primary => text::Appearance {
                color: Some(self.colors.text),
            },
        }
    }
}

#[derive(Default)]
pub enum TextInput {
    #[default]
    Primary,
}

impl text_input::StyleSheet for Theme {
    type Style = TextInput;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        match style {
            TextInput::Primary => text_input::Appearance {
                background: Background::Color(self.colors.background),
                border_radius: 4.0,
                border_width: 1.0,
                border_color: self.colors.primary,
                icon_color: self.colors.primary,
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style) //TODO
    }

    #[allow(unused_variables)]
    fn disabled_color(&self, style: &Self::Style) -> iced::Color {
        Default::default()
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        match style {
            TextInput::Primary => text_input::Appearance {
                background: Background::Color(self.colors.background),
                border_radius: 4.0,
                border_width: 1.0,
                border_color: self.colors.accent,
                // XXX Not currently displayed in application.
                icon_color: self.colors.accent,
            },
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        match style {
            TextInput::Primary => self.colors.text, //TODO lightest
        }
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
