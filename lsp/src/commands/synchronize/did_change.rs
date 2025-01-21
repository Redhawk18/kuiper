use crate::LspSnafu;

use async_lsp::{
    lsp_types::{DidOpenTextDocumentParams, TextDocumentItem, Url},
    LanguageServer, ServerSocket,
};
use snafu::ResultExt;
use std::path::PathBuf;

pub async fn did_change() -> Result<ServerSocket, crate::Error> {
    todo!()
}
