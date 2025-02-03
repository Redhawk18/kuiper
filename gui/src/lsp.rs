use iced::Task;
use tracing::debug;

use kuiper_lsp::Message;

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
            kuiper_lsp::Synchronize::DidOpen => todo!(),
        },
    };
    Task::none()
}
