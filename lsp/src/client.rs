use async_lsp::{
    concurrency::ConcurrencyLayer,
    lsp_types::{
        notification::{Progress, PublishDiagnostics, ShowMessage},
        ClientCapabilities, DidOpenTextDocumentParams, InitializeParams, InitializedParams,
        NumberOrString, ProgressParamsValue, TextDocumentItem, Url, WindowClientCapabilities,
        WorkDoneProgress,
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

    pub async fn did_close() -> Result<ServerSocket, crate::Error> {
        todo!()
    }

    pub async fn did_change() -> Result<ServerSocket, crate::Error> {
        todo!()
    }
    pub async fn did_open(&mut self, text: String, path: PathBuf) -> Result<(), crate::Error> {
        let uri = Url::from_file_path(path).unwrap();

        self.socket
            .did_open(DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri,
                    language_id: "rust".into(),
                    version: 0,
                    text,
                },
            })
            .context(LspSnafu)?;

        Ok(())
    }
}
