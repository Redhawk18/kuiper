use crate::gui::theme::shade::Shade;

/// Holds infomation about the post-processed colors at the highest level
/// The result of Palette mixed with Shade
#[derive(Clone)]
pub struct Colors {
    pub accent: Shade,
    pub background: Shade,
    pub primary: Shade,
    pub secondary: Shade,
    pub text: Shade,
}
