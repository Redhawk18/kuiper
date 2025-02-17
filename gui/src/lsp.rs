use iced::Task;
use tracing::debug;

use kuiper_lsp::Message;

// this file should just be deleted tbh
// for now I'll keep it, because you never know what you'll need.

pub fn update(
    lsp_connection: &mut Option<kuiper_lsp::Connection>,
    message: Message,
) -> Task<Message> {
    match message {
        Message::Initialized(connection) => {
            debug!("Language Server Protocol connection successful.");
            *lsp_connection = Some(connection)
        }
        Message::Shutdown => *lsp_connection = None,
        _ => {}
    };
    Task::none()
}
