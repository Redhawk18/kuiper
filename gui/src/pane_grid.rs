use iced::{
    widget::pane_grid::{Axis, DragEvent, Pane, ResizeEvent, TitleBar},
    widget::{button, column, pane_grid, row, text_editor},
    Alignment, Element, Fill, Renderer, Task, Theme,
};
use iced_aw::core::icons::nerd::{icon_to_text, Nerd};
use kuiper_lsp::client::LSPClient;

mod tab_bar;

use crate::{
    buffer::Buffer,
    style::{pane_active, pane_inactive, title_bar_active, title_bar_inactive},
    Key, Map,
};

#[derive(Debug, Clone)]
pub enum Message {
    PaneClicked(Pane),
    PaneClosed(Pane),
    PaneDragged(DragEvent),
    PaneResized(ResizeEvent),
    PaneSplit(Axis, Pane),

    TabBar(Pane, tab_bar::Message),
    Editor(Pane, text_editor::Action),
}

/// All of the panes in the application.
pub struct Panes {
    active_pane: Pane,
    panes: pane_grid::State<PaneState>,
}

/// The state of a given pane.
#[derive(Default, Clone)]
pub struct PaneState {
    active_buffer: usize,
    open_buffers: Vec<Key>,
}

impl PaneState {
    pub fn active_buffer(&self) -> &usize {
        &self.active_buffer
    }

    pub fn active_buffer_key(&self) -> Option<&Key> {
        self.open_buffers.get(self.active_buffer)
    }

    pub fn insert_buffer(&mut self, key: Key) {
        self.open_buffers.push(key);
        self.active_buffer = self.open_buffers.len() - 1;
    }
}

impl Panes {
    pub fn active_pane(&self) -> Option<&PaneState> {
        self.panes.get(self.active_pane)
    }

    pub fn active_pane_mut(&mut self) -> Option<&mut PaneState> {
        self.panes.get_mut(self.active_pane)
    }

    pub fn active_buffer_key(&self) -> Option<&Key> {
        self.active_pane().and_then(|pane| pane.active_buffer_key())
    }

    pub fn view<'a>(&'a self, map: &'a Map) -> Element<'a, Message, Theme, Renderer> {
        let active = self.active_pane;

        pane_grid(&self.panes, move |pane, state, _is_maximized| {
            let active = active != pane;

            pane_grid::Content::new(body(pane, state.active_buffer(), &state.open_buffers, map))
                .style(if active { pane_active } else { pane_inactive })
                .title_bar(title_bar(active, pane))
        })
        .on_click(Message::PaneClicked)
        .on_drag(Message::PaneDragged)
        .on_resize(10, Message::PaneResized)
        .spacing(15)
        .into()
    }

    pub fn update<'a>(
        &'a mut self,
        message: Message,
        data: &'a mut Map,
        client: &'a mut Option<LSPClient>,
    ) -> Task<Message> {
        match message {
            Message::TabBar(pane_id, message) => match message {
                // Here we could also use something like `tab_bar::update(/* ...
                // */) -> Option<Action>` and match on the result to handle any
                // actions, but since the tab bar is so simple it's shorter and
                // easier to just handle "actions" here directly.
                tab_bar::Message::Selected(buffer_id) => {
                    self.panes.get_mut(pane_id).map(|pane| {
                        pane.active_buffer = buffer_id;
                    });
                }
                tab_bar::Message::Closed(buffer_id) => {
                    self.panes.get_mut(pane_id).map(|pane| {
                        pane.active_buffer = pane.active_buffer.saturating_sub(1);
                        pane.open_buffers.remove(buffer_id);
                    });
                }
            },
            Message::PaneClicked(pane_id) => self.active_pane = pane_id,
            Message::PaneClosed(pane_id) => {
                if let Some((_, sibling)) = self.panes.close(pane_id) {
                    self.active_pane = sibling;
                }
            }
            Message::PaneDragged(pane_grid::DragEvent::Dropped {
                pane: pane_id,
                target,
            }) => {
                self.panes.drop(pane_id, target);
            }
            Message::PaneDragged(_) => {}
            Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            Message::PaneSplit(axis, pane_id) => {
                if let Some(state) = self.panes.get(pane_id) {
                    if let Some((new_pane_id, _)) = self.panes.split(axis, pane_id, state.clone()) {
                        self.active_pane = new_pane_id;
                    }
                }
            }
            Message::Editor(pane_id, action) => {
                if let Some(buffer) = self
                    .panes
                    .get(pane_id)
                    .map(|pane| data.get_mut(pane.open_buffers[pane.active_buffer]))
                    .flatten()
                {
                    match buffer {
                        Buffer::File(file_buffer) => {
                            file_buffer.content.perform(action);
                            tracing::warn!("LSP client update not implemented");
                            if let Some(_client) = client {
                                if let Some(_path) = &file_buffer.path {
                                    // return Task::perform(
                                    // synchronize(path, client.clone().socket),
                                    // |x| {
                                    // Message::LanguageServer(LanguageServer::Syncronize(
                                    // x,
                                    // ))
                                    // },
                                    // );
                                }
                            }
                        }
                    }
                }
            }
        }

        Task::none()
    }
}

fn title_bar<'a>(active: bool, pane: Pane) -> TitleBar<'a, Message, Theme, Renderer> {
    TitleBar::new(Element::from(
        row!(
            button(icon_to_text(Nerd::SplitHorizontal))
                .on_press(Message::PaneSplit(pane_grid::Axis::Vertical, pane)),
            button(icon_to_text(Nerd::SplitVertical))
                .on_press(Message::PaneSplit(pane_grid::Axis::Horizontal, pane)),
            button(icon_to_text(Nerd::X)).on_press(Message::PaneClosed(pane)),
        )
        .align_y(Alignment::Center),
    ))
    .style(if active {
        title_bar_active
    } else {
        title_bar_inactive
    })
    .into()
}

fn body<'a>(
    pane: Pane,
    active: &'a usize,
    visible: &'a Vec<Key>,
    data: &'a Map,
) -> Element<'a, Message, Theme, Renderer> {
    // create a reusable iterator for visible buffers
    let visible_buffers = || visible.iter().filter_map(|key| data.get(*key));

    // early return if no buffers
    if visible_buffers().next().is_none() {
        return column![].into();
    }

    let active_buffer = visible.get(*active).and_then(|key| data.get(*key));

    match active_buffer {
        Some(Buffer::File(file_buffer)) => column![
            tab_bar::view(*active, visible_buffers()).map(move |msg| Message::TabBar(pane, msg)),
            text_editor(&file_buffer.content)
                .height(Fill)
                .on_action(move |action| Message::Editor(pane, action))
        ]
        .spacing(1)
        .into(),
        None => column![].into(),
    }
}

impl Default for Panes {
    fn default() -> Self {
        let (state, pane) = pane_grid::State::new(PaneState::default());
        Self {
            active_pane: pane,
            panes: state,
        }
    }
}
