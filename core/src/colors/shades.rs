use palette::{rgb::Rgb, DarkenAssign, FromColor, LightenAssign, Okhsl, Srgb};

#[derive(Clone)]
pub struct Shades {
    pub default: Rgb,
    pub dark: Rgb,
    pub darker: Rgb,
    pub darkest: Rgb,
    pub light: Rgb,
    pub lighter: Rgb,
    pub lightest: Rgb,
}

impl Shades {
    pub fn new(color: Rgb) -> Self {
        Self {
            default: color,
            dark: darken(color, 0.25),
            darker: darken(color, 0.50),
            darkest: darken(color, 0.75),
            light: lighten(color, 0.25),
            lighter: lighten(color, 0.50),
            lightest: lighten(color, 0.75),
        }
    }
}

fn to_hsl(color: Rgb) -> Okhsl {
    let mut hsl = Okhsl::from_color(color);
    if hsl.saturation.is_nan() {
        hsl.saturation = Okhsl::max_saturation();
    }

    hsl
}

fn from_hsl(hsl: Okhsl) -> Rgb {
    Srgb::from_color(hsl)
}

fn darken(color: Rgb, amount: f32) -> Rgb {
    let mut hsl = to_hsl(color);

    hsl.darken_fixed_assign(amount);

    from_hsl(hsl)
}

fn lighten(color: Rgb, amount: f32) -> Rgb {
    let mut hsl = to_hsl(color);

    hsl.lighten_fixed_assign(amount);

    from_hsl(hsl)
}
