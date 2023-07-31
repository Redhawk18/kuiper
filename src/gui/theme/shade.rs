use iced::Color;

/// provides post-processing to Palette's raw colors
#[derive(Clone)]
pub struct Shade {
    pub base: Color,
    //TODO add shades for light and dark
}

impl Shade {
    pub fn new(base: Color) -> Self {
        //TODO modify light and dark shades
        Self { base }
    }
}
