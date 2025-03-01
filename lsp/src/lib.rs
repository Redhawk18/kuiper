pub mod client;

use futures::{
    SinkExt,
    channel::mpsc::{self, Receiver},
    stream::{Stream, StreamExt},
};
use iced_runtime::futures::stream::channel;
use snafu::Snafu;
use std::{path::PathBuf, sync::Arc};
use tracing::warn;

/// The sender to commincate fast and thread safe with the [`client::LanguageServerProtocolClient`].
#[derive(Debug, Clone)]
pub struct Connection(mpsc::Sender<Message>);

#[derive(Debug, Clone, Snafu)]
pub enum Error {
    Lsp {
        #[snafu(source(from(async_lsp::Error, Arc::new)))]
        source: Arc<async_lsp::Error>,
    },
}

/// One message for each action the [`client::LanguageServerProtocolClient`] can take according to the [specification](https://microsoft.github.io/language-server-protocol/).
#[derive(Debug, Clone)]
pub enum Message {
    Initialized(Connection),
    Shutdown,
    Synchronize(Synchronize),
}

/// Nested all messages that are about file synchronization.
#[derive(Debug, Clone)]
pub enum Synchronize {
    DidChange(String, PathBuf),
    DidClose,
    DidOpen(String, PathBuf),
    DidSave(String, PathBuf),
    WillSave(PathBuf),
}

/// How the internals of the stream are handled. This enum never leaves the stream and never is
/// touched by the gui. It is self managed state by the [`client`] stream.
#[derive(Debug, Default)]
enum State {
    Connected(client::LanguageServerProtocolClient, Receiver<Message>),
    #[default]
    Disconnected,
}

impl Connection {
    pub fn send(&mut self, message: Message) {
        self.0
            .try_send(message)
            .expect("Send message to language server protocol server.");
    }
}

impl From<Synchronize> for Message {
    fn from(value: Synchronize) -> Self {
        Message::Synchronize(value)
    }
}

/// Starts the [`client::LanguageServerProtocolClient`] [`Stream`] which is likely to last the entire program lifetime. The
/// reason for this is these server processes are often very expensive to start, and the gain from
/// reclaiming the resources will likely not matter much to the end user.
pub fn client() -> impl Stream<Item = Message> {
    const CHANNEL_SIZE: usize = 1024;
    channel(CHANNEL_SIZE, async |mut output| {
        let mut state = State::default();

        loop {
            match state {
                State::Connected(ref mut client, ref mut receiver) => {
                    if let Some(message) = receiver.next().await {
                        match message {
                            Message::Initialized(_sender) => {}
                            Message::Shutdown => {
                                let _ = client.shutdown().await;
                            }
                            Message::Synchronize(synchronize) => match synchronize {
                                Synchronize::DidChange(string, path) => {
                                    let _ = client.did_change(string, path).await;
                                }
                                Synchronize::DidClose => todo!(),
                                Synchronize::DidOpen(string, path) => {
                                    let _ = client.did_open(string, path).await;
                                }
                                Synchronize::DidSave(string, path) => {
                                    let _ = client.did_save(string, path);
                                }
                                Synchronize::WillSave(path) => {
                                    let _ = client.will_save(path);
                                }
                            },
                        }
                    }
                }
                State::Disconnected => {
                    match client::LanguageServerProtocolClient::initialize().await {
                        Ok(client) => {
                            let (sender, reciever) = mpsc::channel(CHANNEL_SIZE);
                            let _ = output.send(Message::Initialized(Connection(sender))).await;
                            state = State::Connected(client, reciever);
                        }
                        Err(e) => {
                            warn!("{e}");
                            let _ = output.send(Message::Shutdown).await;
                        }
                    }
                }
            }
        }
    })
}
