pub use async_lsp::ServerSocket;

#[derive(Clone)]
pub struct LSPClient {
    pub socket: ServerSocket,
}
impl LSPClient {
    pub fn new(socket: ServerSocket) -> Self {
        Self { socket }
    }
}
