use iced::widget::container;
use iced::Theme;

pub fn title_bar_active(theme: &Theme) -> container::Appearance {
    let palette = theme.extended_palette();

    container::Appearance {
        text_color: Some(palette.background.strong.text),
        background: Some(palette.background.strong.color.into()),
        ..Default::default()
    }
}

pub fn title_bar_inactive(theme: &Theme) -> container::Appearance {
    let palette = theme.extended_palette();

    container::Appearance {
        text_color: Some(palette.primary.strong.text),
        background: Some(palette.primary.strong.color.into()),
        ..Default::default()
    }
}

pub fn pane_active(theme: &Theme) -> container::Appearance {
    let palette = theme.extended_palette();

    container::Appearance {
        background: Some(palette.background.weak.color.into()),
        border_width: 2.0,
        border_color: palette.background.strong.color,
        ..Default::default()
    }
}

pub fn pane_inactive(theme: &Theme) -> container::Appearance {
    let palette = theme.extended_palette();

    container::Appearance {
        background: Some(palette.background.weak.color.into()),
        border_width: 2.0,
        border_color: palette.primary.strong.color,
        ..Default::default()
    }
}
