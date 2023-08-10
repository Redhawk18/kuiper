mod colors;
mod pigment;
mod shade;
use colors::Colors;
use pigment::Pigment;
use shade::Shade;

use iced::widget::{button, text, text_input};
use iced::{application, Background};
use iced_aw::{menu, tab_bar};

pub type Renderer = iced::Renderer<Theme>;
pub type Element<'msg, Message> = iced::Element<'msg, Message, Renderer>;

#[derive(Clone)]
pub struct Theme {
    colors: Colors,
}

impl Default for Theme {
    fn default() -> Self {
        let pigment = Pigment::default();

        Theme {
            colors: Colors {
                accent: Shade {
                    default: pigment.accent,
                },
                background: Shade {
                    default: pigment.background,
                },
                primary: Shade {
                    default: pigment.primary,
                },
                secondary: Shade {
                    default: pigment.secondary,
                },
                text: Shade {
                    default: pigment.text,
                },
            },
        }
    }
}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.colors.background.default,
            text_color: self.colors.text.default,
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
                border_color: self.colors.accent.default,
                border_radius: 0.0.into(),
                border_width: 0.0,
                shadow_offset: iced::Vector::default(),
                text_color: self.colors.text.default,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Primary => button::Appearance {
                background: Some(Background::Color(self.colors.accent.default)),
                border_color: self.colors.accent.default,
                border_radius: 8.0.into(),
                border_width: 0.0,
                shadow_offset: iced::Vector::default(),
                text_color: self.colors.text.default,
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.hovered(style)
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
                background: self.colors.background.default,
                border_width: 2.0,
                border_radius: [4.0; 4],
                border_color: self.colors.primary.default,
                background_expand: [0; 4],
                path: self.colors.accent.default,
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
                background: Some(Background::Color(self.colors.background.default)),
                border_color: Some(self.colors.primary.default),
                border_width: 4.0,
                icon_color: self.colors.accent.default,
                tab_label_background: Background::Color(if is_active {
                    self.colors.primary.default
                } else {
                    self.colors.secondary.default
                }),
                tab_label_border_color: self.colors.primary.default,
                tab_label_border_width: 1.0,
                text_color: self.colors.text.default,
            },
        }
    }

    fn hovered(&self, style: Self::Style, _is_active: bool) -> tab_bar::Appearance {
        match style {
            TabBar::Primary => tab_bar::Appearance {
                background: Some(Background::Color(self.colors.background.default)),
                border_color: Some(self.colors.accent.default),
                border_width: 4.0,
                icon_color: self.colors.accent.default,
                tab_label_background: Background::Color(self.colors.primary.default),
                tab_label_border_color: self.colors.accent.default,
                tab_label_border_width: 1.0,
                text_color: self.colors.text.default,
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
                color: Some(self.colors.text.default),
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
                background: Background::Color(self.colors.background.default),
                border_radius: 4.0.into(),
                border_width: 1.0,
                border_color: self.colors.primary.default,
                icon_color: self.colors.primary.default,
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
                background: Background::Color(self.colors.background.default),
                border_radius: 4.0.into(),
                border_width: 1.0,
                border_color: self.colors.accent.default,
                // XXX Not currently displayed in application.
                icon_color: self.colors.accent.default,
            },
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        match style {
            TextInput::Primary => self.colors.text.default, //TODO lightest
        }
    }

    fn selection_color(&self, style: &Self::Style) -> iced::Color {
        match style {
            TextInput::Primary => self.colors.accent.default,
        }
    }

    fn value_color(&self, style: &Self::Style) -> iced::Color {
        match style {
            TextInput::Primary => self.colors.text.default,
        }
    }
}
