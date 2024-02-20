use crate::{widgets::tab_bar, Message, Panes, Tab};

use iced::{
    widget::{
        button,
        pane_grid::{Axis, Content, Pane, PaneGrid, TitleBar},
        row,
        text, //horizontal_space,
    },
    Alignment, //Length,
    Renderer,
};
use iced_aw::graphics::icons::{icon_to_char, Icon};
use slotmap::{DefaultKey, SlotMap};

pub(crate) fn pane_grid<'a>(
    panes: &'a Panes,
    map: &SlotMap<DefaultKey, Tab>,
) -> PaneGrid<'a, Message, Renderer> {
    PaneGrid::new(&panes.data, |pane, state, _is_maximized| {
        // currently this is fine **if** we want all gui elements to be tabs
        Content::new(tab_bar(state.active_tab, &state.get_data(map))).title_bar(title_bar(pane))
    })
    .on_click(Message::PaneClicked)
    .on_drag(Message::PaneDragged)
    .on_resize(10, Message::PaneResized)
    .spacing(15)
}

fn title_bar(pane: Pane) -> TitleBar<'static, Message, Renderer> {
    TitleBar::new(
        row!(
            button(
                text(icon_to_char(Icon::ChevronDoubleRight).to_string()).font(iced_aw::ICON_FONT)
            )
            .on_press(Message::PaneSplit(Axis::Vertical, pane)),
            button(
                text(icon_to_char(Icon::ChevronDoubleDown).to_string()).font(iced_aw::ICON_FONT)
            )
            .on_press(Message::PaneSplit(Axis::Horizontal, pane)),
            button(text(icon_to_char(Icon::X).to_string()).font(iced_aw::ICON_FONT))
                .on_press(Message::PaneClosed(pane)),
        )
        .align_items(Alignment::Center),
    )
}
