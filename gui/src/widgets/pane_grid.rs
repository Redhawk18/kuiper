use crate::{
    style::{pane_active, pane_inactive, title_bar_active, title_bar_inactive},
    widgets::tab_bar,
    Buffer, Message, Panes,
};

use iced::{
    widget::{
        button,
        pane_grid::{Axis, Content, Pane, PaneGrid, TitleBar},
        row, text,
    },
    Alignment, Renderer, Theme,
};
use iced_aw::graphics::icons::{icon_to_char, BootstrapIcon};
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
    .on_click(Message::PaneClicked)
    .on_drag(Message::PaneDragged)
    .on_resize(10, Message::PaneResized)
    .spacing(15)
}

fn title_bar(active: bool, pane: Pane) -> TitleBar<'static, Message, Theme, Renderer> {
    TitleBar::new(
        row!(
            button(
                text(icon_to_char(BootstrapIcon::ChevronDoubleRight).to_string())
                    .font(iced_aw::BOOTSTRAP_FONT)
            )
            .on_press(Message::PaneSplit(Axis::Vertical, pane)),
            button(
                text(icon_to_char(BootstrapIcon::ChevronDoubleDown).to_string())
                    .font(iced_aw::BOOTSTRAP_FONT)
            )
            .on_press(Message::PaneSplit(Axis::Horizontal, pane)),
            button(text(icon_to_char(BootstrapIcon::X).to_string()).font(iced_aw::BOOTSTRAP_FONT))
                .on_press(Message::PaneClosed(pane)),
        )
        .align_items(Alignment::Center),
    )
    .style(if active {
        title_bar_active
    } else {
        title_bar_inactive
    })
}
