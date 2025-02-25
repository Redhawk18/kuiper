use async_lsp::{
    concurrency::ConcurrencyLayer,
    lsp_types::{
        notification::{DidChangeTextDocument, Progress, PublishDiagnostics, ShowMessage},
        ClientCapabilities, DidChangeTextDocumentParams, DidOpenTextDocumentParams,
        DidSaveTextDocumentParams, InitializeParams, InitializedParams, NumberOrString,
        ProgressParamsValue, TextDocumentContentChangeEvent, TextDocumentIdentifier,
        TextDocumentItem, TextDocumentSaveReason, Url, VersionedTextDocumentIdentifier,
        WillSaveTextDocumentParams, WindowClientCapabilities, WorkDoneProgress,
    },
    panic::CatchUnwindLayer,
    router::Router,
    tracing::TracingLayer,
    LanguageServer, ServerSocket,
};
use snafu::ResultExt;
use std::{
    ops::ControlFlow,
    path::{Path, PathBuf},
    process::Stdio,
};
use tokio::{
    process::{Child, Command},
    spawn,
    sync::oneshot,
};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};
use tower::builder::ServiceBuilder;
use tracing::{debug, info, trace};

use crate::LspSnafu;

struct Stop;

const LANGUAGE: &str = "rust";
const TEST_ROOT: &str = ".";

struct ClientState {
    indexed_tx: Option<oneshot::Sender<()>>,
}

#[derive(Debug)]
pub struct LanguageServerProtocolClient {
    /// We want to hold onto the child process to properly kill on drop.
    _process: Child,
    socket: ServerSocket,
}

impl LanguageServerProtocolClient {
    pub async fn initialize() -> Result<Self, crate::Error> {
        let root_dir = Path::new(TEST_ROOT)
            .canonicalize()
            .expect("test root should be valid");

        let (indexed_tx, indexed_rx) = oneshot::channel();

        let (mainloop, mut server) = async_lsp::MainLoop::new_client(|_server| {
            let mut router = Router::new(ClientState {
                indexed_tx: Some(indexed_tx),
            });
            router
            .notification::<Progress>(|this, prog| {
                trace!("{:#?} {:#?}", prog.token, prog.value);
                if matches!(prog.token, NumberOrString::String(s) if s == "rustAnalyzer/Indexing")
                    && matches!(
                        prog.value,
                        ProgressParamsValue::WorkDone(WorkDoneProgress::End(_))
                    )
                {
                    // Sometimes rust-analyzer auto-index multiple times?
                    if let Some(tx) = this.indexed_tx.take() {
                        let _: Result<_, _> = tx.send(());
                    }
                }
                ControlFlow::Continue(())
            })
            .notification::<PublishDiagnostics>(|_, _| ControlFlow::Continue(()))
            .notification::<ShowMessage>(|_, params| {
                debug!("Message {:?}: {}", params.typ, params.message);
                ControlFlow::Continue(())
            })
            .event(|_, _: Stop| ControlFlow::Break(Ok(())));

            ServiceBuilder::new()
                .layer(TracingLayer::default())
                .layer(CatchUnwindLayer::default())
                .layer(ConcurrencyLayer::default())
                .service(router)
        });

        let mut child = Command::new("rust-analyzer")
            .current_dir(&root_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true)
            .spawn()
            .expect("Failed run rust-analyzer");
        let stdout = child.stdout.take().unwrap().compat();
        let stdin = child.stdin.take().unwrap().compat_write();

        let _mainloop_future = spawn(async move { mainloop.run_buffered(stdout, stdin).await });

        // Initialize.
        let root_uri = Url::from_file_path(&root_dir).unwrap();
        let init_ret = server
            .initialize(InitializeParams {
                root_uri: Some(root_uri),
                capabilities: ClientCapabilities {
                    window: Some(WindowClientCapabilities {
                        work_done_progress: Some(true),
                        ..WindowClientCapabilities::default()
                    }),
                    ..ClientCapabilities::default()
                },
                ..InitializeParams::default()
            })
            .await
            .unwrap();
        info!("Initialized: {init_ret:?}");
        server.initialized(InitializedParams {}).unwrap();

        // Wait until indexed.
        indexed_rx.await.unwrap();

        Ok(Self {
            _process: child,
            socket: server,
        })
    }

    /// Shutdowns server, this method is not run on process termination.
    /// Only on user requested process termination.
    pub async fn shutdown(&mut self) -> Result<(), crate::Error> {
        info!("Language Server Protocol Client shutting down.");
        self.socket.shutdown(()).await.context(LspSnafu)?;
        self.socket.exit(()).context(LspSnafu)?;
        self.socket.emit(Stop).context(LspSnafu)?;
        trace!("Language Server Protocol Client shutdown complete.");
        Ok(())
    }

    pub async fn did_close() -> Result<(), crate::Error> {
        todo!()
    }

    pub async fn did_change(&mut self, changed: String, path: PathBuf) -> Result<(), crate::Error> {
        let uri = Url::from_file_path(path).unwrap();

        trace!("didchange");
        self.socket
            .did_change(DidChangeTextDocumentParams {
                text_document: VersionedTextDocumentIdentifier {
                    uri,
                    // TODO we have to increase the version number before we do this
                    version: 1,
                },
                content_changes: vec![TextDocumentContentChangeEvent {
                    // OPTIMIZATION we need to send only the sections that have changed.
                    // range is the starting..ending lines that have changed.
                    // text is the text from the sections that have changed.
                    // range_length is deprecated, keep as none.
                    range: None,
                    range_length: None,
                    text: changed,
                }],
            })
            .context(LspSnafu)?;
        trace!("didchanged");

        Ok(())
    }

    pub async fn did_open(&mut self, text: String, path: PathBuf) -> Result<(), crate::Error> {
        let uri = Url::from_file_path(path).unwrap();

        self.socket
            .did_open(DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri,
                    language_id: LANGUAGE.into(),
                    version: 0,
                    text,
                },
            })
            .context(LspSnafu)?;

        Ok(())
    }

    pub async fn did_save(&mut self, text: String, path: PathBuf) -> Result<(), crate::Error> {
        let uri = Url::from_file_path(path).unwrap();

        self.socket
            .did_save(DidSaveTextDocumentParams {
                text_document: TextDocumentIdentifier { uri },
                text: Some(text),
            })
            .context(LspSnafu)?;

        Ok(())
    }

    pub async fn will_save(&mut self, path: PathBuf) -> Result<(), crate::Error> {
        let uri = Url::from_file_path(path).unwrap();

        self.socket
            .will_save(WillSaveTextDocumentParams {
                text_document: TextDocumentIdentifier { uri },
                reason: TextDocumentSaveReason::MANUAL,
            })
            .context(LspSnafu)?;

        Ok(())
    }
}
