use iced::Task;

use kuiper_lsp::client::LSPClient;

#[derive(Debug)]
pub enum Message {
    Initalize(Result<LSPClient, kuiper_lsp::Error>),
    Shutdown,
    Syncronize(Syncronize),
}

#[derive(Debug)]
pub enum Syncronize {
    DidClose,
    DidChange,
    DidOpen(Result<(), kuiper_lsp::Error>),
}

pub fn update(lsp_client: &mut Option<LSPClient>, message: Message) -> Task<Message> {
    match message {
        Message::Initalize(result) => match result {
            Ok(server) => {
                *lsp_client = Some(server);
            }
            Err(e) => {
                eprintln!("Error initializing language server: {}", e);
                return Task::none();
            }
        },
        Message::Shutdown => {
            if let Some(client) = lsp_client {
                // TODO it doesn't currently work lmao
                // this is a future, but we're in a non-async block
                client.shutdown();
            }
        }
        Message::Syncronize(sync) => {
            if let Some(_client) = lsp_client {
                match sync {
                    Syncronize::DidClose => {
                        eprintln!("DidClose not implemented");
                    }
                    Syncronize::DidChange => {
                        eprintln!("DidChange not implemented");
                    }
                    Syncronize::DidOpen(_) => {
                        eprintln!("DidOpen not implemented");
                    }
                }
            }
        }
    }

    Task::none()
}
