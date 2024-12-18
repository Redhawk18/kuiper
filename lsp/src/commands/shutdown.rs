use async_lsp::{LanguageServer, ServerSocket};
use snafu::ResultExt;

use crate::{Error, LspSnafu};

struct Stop;

pub async fn shutdown(mut server: ServerSocket) -> Result<(), crate::Error> {
    server.shutdown(()).await.context(LspSnafu);
    server.exit(()).context(LspSnafu);
    server.emit(Stop).context(LspSnafu);
    // TODO
    // mainloop_fut.await.unwrap();

    Ok(())
}
