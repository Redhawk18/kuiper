use async_lsp::{LanguageServer, ServerSocket};
use snafu::ResultExt;
use tracing::{debug, info};

use crate::{Error, LspSnafu};

struct Stop;

pub async fn shutdown(mut server: ServerSocket) -> Result<(), crate::Error> {
    info!("Language Server Protocol Client shutting down.");

    server.shutdown(()).await.context(LspSnafu);
    server.exit(()).context(LspSnafu);
    server.emit(Stop).context(LspSnafu);
    // TODO
    // mainloop_fut.await.unwrap();

    debug!("Language Server Protocol Client shutdown complete.");

    Ok(())
}
