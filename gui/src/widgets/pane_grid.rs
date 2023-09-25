use crate::{
    theme::{Container, Renderer},
    widgets::tab_bar,
    Message, Panes, Tab,
};

use iced::{
    widget::{
        button,
        pane_grid::{Axis, Content, Pane, PaneGrid, TitleBar},
        row,
        text, //horizontal_space,
    },
    Alignment, //Length,
};
use iced_aw::graphics::icons::{icon_to_char, Icon};
use slotmap::{DefaultKey, SlotMap};

pub(crate) fn pane_grid<'a>(
    panes: &'a Panes,
    map: &SlotMap<DefaultKey, Tab>,
) -> PaneGrid<'a, Message, Renderer> {
    PaneGrid::new(&panes.data, |pane, state, _is_maximized| {
        let active = panes.active == pane;

        // currently this is fine **if** we want all gui elements to be tabs
        Content::new(tab_bar(state.active_tab, &state.get_data(map)))
            .style(Container::PaneGridContent(active))
            .title_bar(title_bar(active, pane))
    })
    .on_click(Message::PaneClicked)
    .on_drag(Message::PaneDragged)
    .on_resize(10, Message::PaneResized)
    .spacing(15)
}

fn title_bar(active: bool, pane: Pane) -> TitleBar<'static, Message, Renderer> {
    TitleBar::new(
        row!(
            //horizontal_space(Length::Fill),
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
    .style(Container::PaneGridTitleBar(active))
}
