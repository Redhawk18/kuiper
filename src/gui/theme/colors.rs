use crate::gui::theme::shades::Shades;

/// Holds infomation about the post-processed colors at the highest level
/// The result of Palette mixed with Shade
#[derive(Clone)]
pub struct Colors {
    pub accent: Shades,
    pub background: Shades,
    pub primary: Shades,
    pub secondary: Shades,
    pub text: Shades,
}
