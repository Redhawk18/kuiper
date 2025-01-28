use async_lsp::{
    concurrency::ConcurrencyLayer,
    lsp_types::{
        notification::{Progress, PublishDiagnostics, ShowMessage},
        ClientCapabilities, InitializeParams, InitializedParams, NumberOrString,
        ProgressParamsValue, Url, WindowClientCapabilities, WorkDoneProgress,
    },
    panic::CatchUnwindLayer,
    router::Router,
    tracing::TracingLayer,
    LanguageClient, LanguageServer, ResponseError, ServerSocket,
};
use std::{borrow::Borrow, ops::ControlFlow, path::Path, process::Stdio};
use tokio::{process::Command, sync::oneshot, task::spawn};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};
use tower::builder::ServiceBuilder;
use tracing::{debug, info, trace};

use crate::client::LSPClient;

const TEST_ROOT: &str = ".";

struct ClientState {
    indexed_tx: Option<oneshot::Sender<()>>,
}

struct Stop;

impl LanguageClient for ClientState {
    type Error = ResponseError;
    type NotifyResult = ControlFlow<async_lsp::Result<()>>;
}

pub async fn initialize() -> Result<LSPClient, crate::Error> {
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
        // .kill_on_drop(true)
        .spawn()
        .expect("Failed run rust-analyzer");
    let stdout = child.stdout.take().unwrap().compat();
    let stdin = child.stdin.take().unwrap().compat_write();

    let _mainloop_fut = spawn(async move { mainloop.run_buffered(stdout, stdin).await });

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

    // Synchronize documents.
    // these need to be in a command that runs consisntly
    // let file_uri = Url::from_file_path(root_dir.join("src/main.rs")).unwrap();
    // let text = tokio::fs::read_to_string(file_uri.path()).await.unwrap();
    // server
    //     .did_open(DidOpenTextDocumentParams {
    //         text_document: TextDocumentItem {
    //             uri: file_uri.clone(),
    //             language_id: "rust".into(),
    //             version: 0,
    //             text: text.clone(),
    //         },
    //     })
    //     .unwrap();

    // Wait until indexed.
    indexed_rx.await.unwrap();

    //        // Shutdown.
    //        // server.shutdown(()).await.unwrap();
    //        // server.exit(()).unwrap();

    //        // server.emit(Stop).unwrap();
    //        // mainloop_fut.await.unwrap();
    //        //
    //        //

    // Ok((mainloop_fut, child, server))

    let client = LSPClient::new(child, server);
    Ok(client)
}
