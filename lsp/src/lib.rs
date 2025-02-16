pub mod client;

use futures::{
    channel::mpsc::{self, Receiver},
    stream::{Stream, StreamExt},
    SinkExt,
};
use iced_runtime::futures::stream::channel;
use snafu::Snafu;
use std::{path::PathBuf, sync::Arc};
use tracing::{error, warn};

const CHANNEL_SIZE: usize = 1024;

#[derive(Debug, Clone, Snafu)]
pub enum Error {
    Lsp {
        #[snafu(source(from(async_lsp::Error, Arc::new)))]
        source: Arc<async_lsp::Error>,
    },
}

#[derive(Debug, Default)]
pub enum State {
    Connected(client::LSPClient, Receiver<Message>),
    #[default]
    Disconnected,
}

#[derive(Debug, Clone)]
pub struct Connection(mpsc::Sender<Message>);

#[derive(Debug, Clone)]
pub enum Message {
    Initialized(Connection),
    Shutdown,
    Synchronize(Synchronize),
}

#[derive(Debug, Clone)]
pub enum Synchronize {
    DidChange,
    DidClose,
    DidOpen(PathBuf),
}

impl Connection {
    pub fn send(&mut self, message: Message) {
        self.0
            .try_send(message)
            .expect("Send message to echo server");
    }
}

pub fn client() -> impl Stream<Item = Message> {
    channel(CHANNEL_SIZE, |mut output| async move {
        let mut state = State::default();

        loop {
            match state {
                State::Connected(ref mut client, ref mut receiver) => {
                    if let Some(message) = receiver.next().await {
                        match message {
                            Message::Initialized(sender) => {
                                error!("we are init");
                            }
                            Message::Shutdown => {
                                let _ = client.shutdown().await;
                            }
                            Message::Synchronize(synchronize) => match synchronize {
                                Synchronize::DidChange => todo!(),
                                Synchronize::DidClose => todo!(),
                                Synchronize::DidOpen(path) => {
                                    error!("lsp/src/lib.rs 75");
                                    let _ = client.did_open(path).await;
                                }
                            },
                        }
                    }
                }
                State::Disconnected => match client::LSPClient::initialize().await {
                    Ok(client) => {
                        let (sender, reciever) = mpsc::channel(CHANNEL_SIZE);
                        let _ = output.send(Message::Initialized(Connection(sender))).await;
                        state = State::Connected(client, reciever);
                    }
                    Err(e) => {
                        warn!("{e}");
                        let _ = output.send(Message::Shutdown).await;
                    }
                },
            }
        }
    })
}
