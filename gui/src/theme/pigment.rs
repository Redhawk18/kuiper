use iced::Color;

/// raw hex colors from the theme files
#[derive(Clone, Copy)]
pub struct Pigment {
    pub accent: Color,
    pub background: Color,
    pub primary: Color,
    pub secondary: Color,
    pub text: Color,
}

impl Pigment {
    pub fn new(is_light: bool) -> Self {
        Self {
            accent: if is_light {
                hex_to_color("#f8073b").unwrap()
            } else {
                hex_to_color("#928fd6").unwrap()
            },
            background: if is_light {
                hex_to_color("#fafafa").unwrap()
            } else {
                hex_to_color("#050505").unwrap()
            },
            primary: if is_light {
                hex_to_color("#f9682f").unwrap()
            } else {
                hex_to_color("#2a4b74").unwrap()
            },
            secondary: if is_light {
                hex_to_color("#fef2cd").unwrap()
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

fn hex_to_color(hex: &str) -> Option<Color> {
    if hex.len() == 7 {
        let hash = &hex[0..1];
        let r = u8::from_str_radix(&hex[1..3], 16);
        let g = u8::from_str_radix(&hex[3..5], 16);
        let b = u8::from_str_radix(&hex[5..7], 16);

        return match (hash, r, g, b) {
            ("#", Ok(r), Ok(g), Ok(b)) => Some(Color {
                r: r as f32 / 255.0,
                g: g as f32 / 255.0,
                b: b as f32 / 255.0,
                a: 1.0,
            }),
            _ => None,
        };
    }

    None
}
