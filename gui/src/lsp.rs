use iced::Task;
use tracing::{debug, error};

use kuiper_lsp::Message;

// this file should just be deleted tbh

pub fn update(
    lsp_connection: &mut Option<kuiper_lsp::Connection>,
    message: Message,
) -> Task<Message> {
    match message {
        Message::Initialized(connection) => {
            debug!("Language Server Protocol connection successful.");
            *lsp_connection = Some(connection)
        }
        Message::Shutdown => todo!(),
        Message::Synchronize(synchronize) => match synchronize {
            kuiper_lsp::Synchronize::DidOpen(_path) => {
                error!("gui/lsp.rs");
            }
            kuiper_lsp::Synchronize::DidChange => todo!(),
            kuiper_lsp::Synchronize::DidClose => todo!(),
        },
    };
    Task::none()
}
