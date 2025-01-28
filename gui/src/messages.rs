use crate::file_dialog;

use iced::{
    font,
    widget::{
        pane_grid::{Axis, DragEvent, Pane, ResizeEvent},
        text_editor::Action,
    },
};
use kuiper_lsp::client::LSPClient;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Message {
    FontLoaded(Result<(), font::Error>),

    LanguageServer(LanguageServer),
    Widgets(Widgets),
}

#[derive(Debug)]
pub enum LanguageServer {
    Initalize(Result<LSPClient, kuiper_lsp::Error>),
    Shutdown(),
    Syncronize(Syncronize),
}

#[derive(Debug)]
pub enum Syncronize {
    DidClose,
    DidChange,
    DidOpen(),
}

#[derive(Debug)]
pub enum Widgets {
    Button(Button),
    PaneGrid(PaneGrid),
    Tab(Tab),
    TextEditor(TextEditor),
}

#[derive(Debug, Clone)]
pub enum Button {
    NewFile,
    OpenFile,
    OpenedFile(Result<(PathBuf, String), file_dialog::Error>),
    OpenFolder,
    OpenedFolder(Result<PathBuf, file_dialog::Error>),
    Save,
    #[allow(dead_code)]
    Saved(Result<(), file_dialog::Error>),
    SaveAs,
    #[allow(dead_code)]
    SavedAs(Result<(), file_dialog::Error>),
    Quit,
}

#[derive(Debug, Clone)]
pub enum PaneGrid {
    PaneClicked(Pane),
    PaneClosed(Pane),
    PaneDragged(DragEvent),
    PaneResized(ResizeEvent),
    PaneSplit(Axis, Pane),
}

#[derive(Debug)]
pub enum Tab {
    TabSelected(usize),
    TabClosed(usize),
}

#[derive(Debug)]
pub enum TextEditor {
    TextEditorUpdate(Action),
}
