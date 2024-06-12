use async_lsp::{
    concurrency::ConcurrencyLayer, panic::CatchUnwindLayer, router::Router, tracing::TracingLayer,
    LanguageClient, LanguageServer, ResponseError,
};
use log::{info, trace, Level};
use lsp_types::{
    notification::{Progress, PublishDiagnostics, ShowMessage},
    ClientCapabilities, DidOpenTextDocumentParams, HoverContents, HoverParams, InitializeParams,
    InitializedParams, MarkupContent, NumberOrString, Position, ProgressParamsValue,
    TextDocumentIdentifier, TextDocumentItem, TextDocumentPositionParams, Url,
    WindowClientCapabilities, WorkDoneProgress, WorkDoneProgressParams,
};
use std::{ops::ControlFlow, path::Path, process::Stdio};
use tokio::{sync::oneshot, process::Command, };
use tokio_util::compat::{ TokioAsyncWriteCompatExt, TokioAsyncReadCompatExt};
use tower::builder::ServiceBuilder;

const TEST_ROOT: &str = ".";

struct ClientState {
    indexed_tx: Option<oneshot::Sender<()>>,
}

struct Stop;

impl LanguageClient for ClientState {
    type Error = ResponseError;
    type NotifyResult = ControlFlow<async_lsp::Result<()>>;
}

pub async fn start_lsp() {
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
                trace!("Message {:?}: {}", params.typ, params.message);
                ControlFlow::Continue(())
            })
            .event(|_, _: Stop| ControlFlow::Break(Ok(())));

        ServiceBuilder::new()
            .layer(TracingLayer::default())
            .layer(CatchUnwindLayer::default())
            .layer(ConcurrencyLayer::default())
            .service(router)
    });

    // tracing_subscriber::fmt()
    //     .with_max_level(Level::Info)
    //     .with_ansi(false)
    //     .with_writer(std::io::stdout)
    //     .init();

    let child = Command::new("rust-analyzer")
        .current_dir(&root_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .kill_on_drop(true)
        .spawn()
        .expect("Failed run rust-analyzer");
    let stdout = child.stdout.unwrap().compat();
    let stdin = child.stdin.unwrap().compat_write();

    let mainloop_fut = tokio::spawn(async move {
        mainloop.run_buffered(stdout, stdin).await.unwrap();
    });

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
    // let file_uri = Url::from_file_path(root_dir.join("src/lib.rs")).unwrap();
    let file_uri = Url::from_file_path(root_dir.join("src/main.rs")).unwrap();
    let text = "fn func() { let var = 1; }";
    server
        .did_open(DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: file_uri.clone(),
                language_id: "rust".into(),
                version: 0,
                text: text.into(),
            },
        })
        .unwrap();

    // Wait until indexed.
    indexed_rx.await.unwrap();

    // Query.
    let var_pos = text.find("var").unwrap();
    let hover = server
        .hover(HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: file_uri },
                position: Position::new(0, var_pos as _),
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
        })
        .await
        .unwrap()
        .unwrap();
    info!("Hover result: {hover:?}");
    assert!(
        matches!(
            hover.contents,
            HoverContents::Markup(MarkupContent { value, .. })
            if value.contains("let var: i32")
        ),
        "should show the type of `var`",
    );

    // Shutdown.
    server.shutdown(()).await.unwrap();
    server.exit(()).unwrap();

    server.emit(Stop).unwrap();
    mainloop_fut.await.unwrap();
}
