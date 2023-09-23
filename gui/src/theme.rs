use blaze_core::colors::Palette;
use dark_light::{detect, Mode};
use iced::{
    application,
    widget::{
        button, container,
        pane_grid::{self, Line},
        text, text_input,
    },
    Background,
};
use iced_aw::{menu, tab_bar};

pub type Renderer = iced::Renderer<Theme>;
pub type Element<'msg, Message> = iced::Element<'msg, Message, Renderer>;

#[derive(Clone)]
pub struct Theme {
    is_light: bool,
    palette: Palette,
}

impl Default for Theme {
    fn default() -> Self {
        let is_light = match detect() {
            Mode::Light | Mode::Default => true,
            Mode::Dark => false,
        };

        Self {
            is_light,
            palette: Palette::new(is_light),
        }
    }
}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.palette.background.default.into(),
            text_color: self.palette.text.default.into(),
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
                border_color: self.palette.accent.default.into(),
                border_radius: 0.0.into(),
                border_width: 0.0,
                shadow_offset: iced::Vector::default(),
                text_color: self.palette.text.default.into(),
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Primary => button::Appearance {
                background: Some(Background::Color(self.palette.accent.default.into())),
                border_color: self.palette.accent.default.into(),
                border_radius: 8.0.into(),
                border_width: 0.0,
                shadow_offset: iced::Vector::default(),
                text_color: self.palette.text.default.into(),
            },
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.hovered(style)
    }

    //fn disabled(&self, style: &Self::Style) -> button::Appearance {} //todo
}

#[derive(Default)]
pub enum Container {
    #[default]
    Primary,
    PaneGridContent(bool),
    PaneGridTitleBar(bool),
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Primary => container::Appearance {
                ..Default::default()
            },
            Container::PaneGridContent(active) => container::Appearance {
                text_color: Some(self.palette.text.default.into()),
                border_color: if *active {
                    self.palette.accent.default.into()
                } else {
                    self.palette.primary.default.into()
                },
                background: Some(Background::Color(self.palette.background.default.into())),
                border_width: 1.0,
                border_radius: [4.0; 4].into(),
            },
            Container::PaneGridTitleBar(active) => container::Appearance {
                text_color: Some(self.palette.text.default.into()),
                border_color: if *active {
                    self.palette.accent.default.into()
                } else {
                    self.palette.primary.default.into()
                },
                background: Some(Background::Color(self.palette.background.default.into())),
                border_width: 1.0,
                border_radius: [4.0, 4.0, 0.0, 0.0].into(),
            },
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

    fn appearance(&self, style: &Self::Style) -> menu::Appearance {
        match style {
            Menu::Primary => menu::Appearance {
                background: self.palette.background.default.into(),
                border_color: self.palette.primary.default.into(),
                border_width: 2.0,
                border_radius: [4.0; 4],
                background_expand: [0; 4],
                path: self.palette.accent.default.into(),
            },
        }
    }
}

#[derive(Default)]
pub enum PaneGrid {
    #[default]
    Primary,
}

impl pane_grid::StyleSheet for Theme {
    type Style = PaneGrid;

    fn hovered_region(&self, style: &Self::Style) -> pane_grid::Appearance {
        match style {
            PaneGrid::Primary => pane_grid::Appearance {
                background: Background::Color(self.palette.secondary.default.into()),
                border_width: 3.0,
                border_color: self.palette.primary.default.into(),
                border_radius: [4.0; 4].into(),
            },
        }
    }

    fn picked_split(&self, style: &Self::Style) -> Option<pane_grid::Line> {
        match style {
            PaneGrid::Primary => Some(Line {
                color: self.palette.accent.default.into(),
                width: 3.0,
            }),
        }
    }

    fn hovered_split(&self, style: &Self::Style) -> Option<pane_grid::Line> {
        match style {
            PaneGrid::Primary => Some(Line {
                color: self.palette.accent.default.into(),
                width: 1.0,
            }),
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

    fn active(&self, style: &Self::Style, is_active: bool) -> tab_bar::Appearance {
        match style {
            TabBar::Primary => tab_bar::Appearance {
                background: Some(Background::Color(self.palette.background.default.into())),
                border_color: Some(self.palette.primary.default.into()),
                border_width: 1.0,
                icon_background: Some(Background::Color(self.palette.background.default.into())),
                icon_border_radius: 0.0.into(),
                icon_color: self.palette.accent.default.into(),
                tab_label_background: Background::Color(if is_active {
                    self.palette.primary.default.into()
                } else {
                    self.palette.secondary.default.into()
                }),
                tab_label_border_color: self.palette.primary.default.into(),
                tab_label_border_width: 1.0,
                text_color: self.palette.text.default.into(),
            },
        }
    }

    fn hovered(&self, style: &Self::Style, _is_active: bool) -> tab_bar::Appearance {
        match style {
            TabBar::Primary => tab_bar::Appearance {
                background: Some(Background::Color(self.palette.background.default.into())),
                border_color: Some(self.palette.accent.default.into()),
                border_width: 1.0,
                icon_background: Some(Background::Color(self.palette.background.light.into())),
                icon_border_radius: 4.0.into(),
                icon_color: self.palette.accent.default.into(),
                tab_label_background: Background::Color(self.palette.primary.default.into()),
                tab_label_border_color: self.palette.accent.default.into(),
                tab_label_border_width: 1.0,
                text_color: self.palette.text.default.into(),
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
                color: Some(self.palette.text.default.into()),
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
                background: Background::Color(self.palette.background.default.into()),
                border_color: self.palette.primary.default.into(),
                border_radius: 4.0.into(),
                border_width: 1.0,
                icon_color: self.palette.primary.default.into(),
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
                background: Background::Color(self.palette.background.default.into()),
                border_color: self.palette.accent.default.into(),
                border_radius: 4.0.into(),
                border_width: 1.0,
                icon_color: self.palette.accent.default.into(),
            },
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        match style {
            TextInput::Primary => {
                if self.is_light {
                    self.palette.text.lightest.into()
                } else {
                    self.palette.text.darkest.into()
                }
            }
        }
    }

    fn selection_color(&self, style: &Self::Style) -> iced::Color {
        match style {
            TextInput::Primary => self.palette.accent.default.into(),
        }
    }

    fn value_color(&self, style: &Self::Style) -> iced::Color {
        match style {
            TextInput::Primary => self.palette.text.default.into(),
        }
    }
}
