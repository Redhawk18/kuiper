use iced::Task;
use tracing::debug;

use kuiper_lsp::Message;

#[macro_export]
macro_rules! send {
    ($self:expr, $inner:expr) => {
        if let Some(client) = &mut $self.lsp_client {
            client.send($inner.into()); // Convert into `kuiper_lsp::Message`
        }
    };
}

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
