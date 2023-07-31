use iced::widget::{button, text, text_input};
use iced::{application, Background};
use iced_aw::menu;
use iced_aw::style::tab_bar; //FIXME https://github.com/iced-rs/iced_aw/issues/151

mod colors;
mod palette;
mod shade;

use colors::Colors;
use palette::Palette;
use shade::Shade;

pub type Renderer = iced::Renderer<Theme>;
pub type Element<'msg, Message> = iced::Element<'msg, Message, Renderer>;

#[derive(Clone)]
pub struct Theme {
    colors: Colors,
}

impl Default for Theme {
    fn default() -> Self {
        let palette = Palette::default();

        Theme {
            colors: Colors {
                accent: Shade {
                    base: palette.accent,
                },
                background: Shade {
                    base: palette.background,
                },
                primary: Shade {
                    base: palette.primary,
                },
                secondary: Shade {
                    base: palette.secondary,
                },
                text: Shade { base: palette.text },
            },
        }
    }
}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.colors.background.base,
            text_color: self.colors.text.base,
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
                border_color: self.colors.accent.base,
                border_radius: 0.0,
                border_width: 0.0,
                shadow_offset: iced::Vector::default(),
                text_color: self.colors.text.base,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Primary => button::Appearance {
                background: Some(Background::Color(self.colors.accent.base)),
                border_color: self.colors.accent.base,
                border_radius: 8.0,
                border_width: 0.0,
                shadow_offset: iced::Vector::default(),
                text_color: self.colors.text.base,
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
                background: self.colors.background.base,
                border_width: 2.0,
                border_radius: [4.0; 4],
                border_color: self.colors.primary.base,
                background_expand: [0; 4],
                path: self.colors.accent.base,
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
                background: Some(Background::Color(self.colors.background.base)),
                border_color: Some(self.colors.primary.base),
                border_width: 4.0,
                icon_color: self.colors.accent.base,
                tab_label_background: Background::Color(if is_active {
                    self.colors.primary.base
                } else {
                    self.colors.secondary.base
                }),
                tab_label_border_color: self.colors.primary.base,
                tab_label_border_width: 1.0,
                text_color: self.colors.text.base,
            },
        }
    }

    fn hovered(&self, style: Self::Style, _is_active: bool) -> tab_bar::Appearance {
        match style {
            TabBar::Primary => tab_bar::Appearance {
                background: Some(Background::Color(self.colors.background.base)),
                border_color: Some(self.colors.accent.base),
                border_width: 4.0,
                icon_color: self.colors.accent.base,
                tab_label_background: Background::Color(self.colors.primary.base),
                tab_label_border_color: self.colors.accent.base,
                tab_label_border_width: 1.0,
                text_color: self.colors.text.base,
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
                color: Some(self.colors.text.base),
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
                background: Background::Color(self.colors.background.base),
                border_radius: 4.0,
                border_width: 1.0,
                border_color: self.colors.primary.base,
                icon_color: self.colors.primary.base,
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
                background: Background::Color(self.colors.background.base),
                border_radius: 4.0,
                border_width: 1.0,
                border_color: self.colors.accent.base,
                // XXX Not currently displayed in application.
                icon_color: self.colors.accent.base,
            },
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        match style {
            TextInput::Primary => self.colors.text.base, //TODO lightest
        }
    }

    fn selection_color(&self, style: &Self::Style) -> iced::Color {
        match style {
            TextInput::Primary => self.colors.accent.base,
        }
    }

    fn value_color(&self, style: &Self::Style) -> iced::Color {
        match style {
            TextInput::Primary => self.colors.text.base,
        }
    }
}
