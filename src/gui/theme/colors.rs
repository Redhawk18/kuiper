//use shade;
use iced::Color;

/// Holds infomation about the post-processed colors at the highest level
/// The result of Palette mixed with Shade
// pub struct Colors {
//     accent: Shade,
//     background: Shade,
//     primary: Shade,
//     secondary: Shade,
//     text: Shade,
// }

#[derive(Clone)]
pub struct Colors {
    pub accent: Color,
    pub background: Color,
    pub primary: Color,
    pub secondary: Color,
    pub text: Color,
}

//TODO review this after get it working in theme.rs
impl Default for Colors {
    fn default() -> Colors {
        Colors {
            accent: hex_to_color("#f8073b").unwrap(),
            background: hex_to_color("#fafafa").unwrap(),
            primary: hex_to_color("#f9682f").unwrap(),
            secondary: hex_to_color("#fef2cd").unwrap(),
            text: hex_to_color("#050505").unwrap(),
        }
    }
}

//TODO REMOVE THIS, JUST FOR TESTING
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
