use async_lsp::{
    lsp_types::{DidOpenTextDocumentParams, TextDocumentItem, Url},
    LanguageServer, ServerSocket,
};
use std::path::PathBuf;

pub async fn synchronize(
    path: PathBuf,
    mut server: ServerSocket,
) -> Result<ServerSocket, async_lsp::Error> {
    //TODO add
    let text = tokio::fs::read_to_string(path.clone()).await.unwrap();
    let uri = Url::from_file_path(path).unwrap();

    match server.did_open(DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri,
            language_id: "rust".into(),
            version: 0,
            text,
        },
    }) {
        Ok(_) => Ok(server.clone()),
        Err(e) => Err(e),
    }
}
