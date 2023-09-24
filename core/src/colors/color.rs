use palette::rgb::Rgb;

#[derive(Clone, Copy)]
pub struct Color {
    pub accent: Rgb,
    pub background: Rgb,
    pub primary: Rgb,
    pub secondary: Rgb,
    pub text: Rgb,
}

impl Color {
    /// Creates a new color from a hex string that is hard coded (TODO),
    /// we have two colors for each value because colors in light and dark mode are completely different.
    #[allow(clippy::if_same_then_else)]
    pub fn new(is_light: bool) -> Self {
        Self {
            accent: if is_light {
                hex_to_color("#3b3795").unwrap()
            } else {
                hex_to_color("#928fd6").unwrap()
            },
            background: if is_light {
                hex_to_color("#fafafa").unwrap()
            } else {
                hex_to_color("#050505").unwrap()
            },
            primary: if is_light {
                hex_to_color("#2a4b74").unwrap()
            } else {
                hex_to_color("#2a4b74").unwrap()
            },
            secondary: if is_light {
                hex_to_color("#dadaf1").unwrap()
            } else {
                hex_to_color("#0e0e25").unwrap()
            },
            text: if is_light {
                hex_to_color("#050505").unwrap()
            } else {
                hex_to_color("#fafafa").unwrap()
            },
        }
    }
}

fn hex_to_color(hex: &str) -> Option<Rgb> {
    if hex.len() == 7 {
        let hash = &hex[0..1];
        let red = u8::from_str_radix(&hex[1..3], 16);
        let green = u8::from_str_radix(&hex[3..5], 16);
        let blue = u8::from_str_radix(&hex[5..7], 16);

        return match (hash, red, green, blue) {
            ("#", Ok(red), Ok(green), Ok(blue)) => Some(Rgb::new(
                red as f32 / 255.0,
                green as f32 / 255.0,
                blue as f32 / 255.0,
            )),
            _ => None,
        };
    }

    None
}
