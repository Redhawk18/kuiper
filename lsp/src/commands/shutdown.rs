use async_lsp::{LanguageServer, ServerSocket};

struct Stop;

pub async fn shutdown(mut server: ServerSocket) -> Result<(), async_lsp::Error> {
    server.shutdown(()).await.unwrap_err();
    server.exit(()).unwrap_err();
    server.emit(Stop).unwrap_err();
    // TODO
    // mainloop_fut.await.unwrap();

    Ok(())
}
