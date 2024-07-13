use crate::{
    style::{pane_active, pane_inactive, title_bar_active, title_bar_inactive},
    widgets::tab_bar,
    Buffer, Message, Panes, Widgets,
};

use iced::{
    widget::{
        button,
        pane_grid::{Axis, Content, Pane, PaneGrid, TitleBar},
        row,
    },
    Alignment, Element, Renderer, Theme,
};
use iced_aw::core::icons::nerd::{icon_to_text, Nerd};
use slotmap::{DefaultKey, SlotMap};

pub(crate) fn pane_grid<'a>(
    panes: &'a Panes,
    map: &'a SlotMap<DefaultKey, Buffer>,
) -> PaneGrid<'a, Message, Theme, Renderer> {
    PaneGrid::new(&panes.data, |pane, state, _is_maximized| {
        let active = panes.active != pane;

        // currently this is fine **if** we want all gui elements to be tabs
        Content::new(tab_bar(state.active_tab_index, &state.get_data(map)))
            .style(if active { pane_active } else { pane_inactive })
            .title_bar(title_bar(active, pane))
    })
    .on_click(|x| Message::Widgets(Widgets::PaneGrid(crate::PaneGrid::PaneClicked(x))))
    .on_drag(|x| Message::Widgets(Widgets::PaneGrid(crate::PaneGrid::PaneDragged(x))))
    .on_resize(10, |x| {
        Message::Widgets(Widgets::PaneGrid(crate::PaneGrid::PaneResized(x)))
    })
    .spacing(15)
}

fn title_bar(active: bool, pane: Pane) -> TitleBar<'static, Message, Theme, Renderer> {
    TitleBar::new(
        Element::from(
            row!(
                button(icon_to_text(Nerd::SplitHorizontal))
                    .on_press(crate::PaneGrid::PaneSplit(Axis::Horizontal, pane)),
                button(icon_to_text(Nerd::SplitVertical))
                    .on_press(crate::PaneGrid::PaneSplit(Axis::Vertical, pane)),
                button(icon_to_text(Nerd::X)).on_press(crate::PaneGrid::PaneClosed(pane)),
            )
            .align_items(Alignment::Center),
        )
        .map(|x| Message::Widgets(Widgets::PaneGrid(x))),
    )
    .style(if active {
        title_bar_active
    } else {
        title_bar_inactive
    })
}
