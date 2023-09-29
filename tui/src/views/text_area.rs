use crate::theme::to_color;

use blaze_core::colors::Palette;
use cursive::{
    theme::{PaletteColor, Theme},
    views::{TextArea, ThemedView},
};

pub fn text_area(theme: Theme) -> ThemedView<TextArea> {
    ThemedView::new(custom_theme(theme), TextArea::new())
}

fn custom_theme(current_theme: Theme) -> Theme {
    let mut theme = current_theme.clone();
    let palette = Palette::new(false);

    //theme.palette[PaletteColor::Secondary] = to_color(palette.secondary.default);
    theme.palette[PaletteColor::Secondary] = to_color(palette.text.default); // TODO remove when cursive is 0.21
    theme.palette[PaletteColor::View] = to_color(palette.text.default); //lib bug, the text should show up as white

    theme
}
