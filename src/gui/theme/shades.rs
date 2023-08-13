use iced::Color;
use palette::rgb::Rgb;
use palette::{DarkenAssign, FromColor, LightenAssign, Okhsl, Srgb};

/// provides post-processing to Pigment's raw colors
#[derive(Clone)]
pub struct Shades {
    pub default: Color,
    pub light: Color,
    pub lighter: Color,
    pub lightest: Color,
    pub dark: Color,
    pub darker: Color,
    pub darkest: Color,
}

impl Shades {
    pub fn new(color: Color) -> Self {
        Self {
            default: color,
            light: lighten(color, 0.25),
            lighter: lighten(color, 0.50),
            lightest: lighten(color, 0.75),
            dark: darken(color, 0.25),
            darker: darken(color, 0.50),
            darkest: darken(color, 0.75),
        }
    }
}

// special thanks to Halloy https://github.com/squidowl/halloy
fn to_hsl(color: Color) -> Okhsl {
    let mut hsl = Okhsl::from_color(Rgb::from(color));
    if hsl.saturation.is_nan() {
        hsl.saturation = Okhsl::max_saturation();
    }

    hsl
}

fn from_hsl(hsl: Okhsl) -> Color {
    Srgb::from_color(hsl).into()
}

// fn alpha(color: Color, alpha: f32) -> Color {
//     Color { a: alpha, ..color }
// }

fn darken(color: Color, amount: f32) -> Color {
    let mut hsl = to_hsl(color);

    hsl.darken_fixed_assign(amount);

    from_hsl(hsl)
}

fn lighten(color: Color, amount: f32) -> Color {
    let mut hsl = to_hsl(color);

    hsl.lighten_fixed_assign(amount);

    from_hsl(hsl)
}

pub fn is_dark(color: Color) -> bool {
    to_hsl(color).lightness < 0.5
}
