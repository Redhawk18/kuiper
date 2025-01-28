pub mod client;
// pub mod commands;

use snafu::Snafu;
use std::sync::Arc;

#[derive(Debug, Clone, Snafu)]
pub enum Error {
    Lsp {
        #[snafu(source(from(async_lsp::Error, Arc::new)))]
        source: Arc<async_lsp::Error>,
    },
}
