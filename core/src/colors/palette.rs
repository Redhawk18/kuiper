use super::{color::Color, shades::Shades};

#[derive(Clone)]
pub struct Palette {
    pub accent: Shades,
    pub background: Shades,
    pub primary: Shades,
    pub secondary: Shades,
    pub text: Shades,
}

impl Palette {
    pub fn new(is_light: bool) -> Self {
        let color = Color::new(is_light);
        Self {
            accent: Shades::new(color.accent),
            background: Shades::new(color.background),
            primary: Shades::new(color.primary),
            secondary: Shades::new(color.secondary),
            text: Shades::new(color.text),
        }
    }
}
