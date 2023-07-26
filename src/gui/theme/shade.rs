use Palette;

/// provides post-processing to Palette's raw colors
pub struct Shade {
    pub base: Palette,

    //TODO add shades for light and dark
}

impl Shade {
    pub fn new(base: Color) -> Self {
        //TODO modify light and dark shades
        Self
        {
            base
        }
    }
}
