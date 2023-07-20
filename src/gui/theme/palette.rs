use iced::Color;

/// raw hex colors from the theme files
pub struct Palette {
    accent: Color,
    background: Color, 
    primary: Color,
    secondary: Color,
    text: Color,
}

impl Default for Palette {
    fn default() -> Palette {
        Palette {
            accent: hex_to_color("#000000").unwrap(),
            background: hex_to_color("#ffffff").unwrap(), 
            primary: hex_to_color("#4685ff").unwrap(),
            secondary: hex_to_color("#f2f2f2").unwrap(),
            text: hex_to_color("#ffb084").unwrap(),
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
