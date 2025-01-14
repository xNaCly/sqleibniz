mod error;

use error::LspError;
use lsp_server::Connection;
use lsp_types::{OneOf, ServerCapabilities};

macro_rules! lsp_log {
    ($literal:literal) => {
        eprintln!("[sqleibniz]: {}", $literal)
    };
}

pub fn start() -> Result<(), LspError> {
    lsp_log!("starting language server");
    let (connection, threads) = Connection::stdio();
    let capabilities = serde_json::to_value(&ServerCapabilities {
        // TODO: add the real thing here (diagnostics, hover, etc)
        definition_provider: Some(OneOf::Left(true)),
        ..Default::default()
    })
    .map_err(|_| "failed to serialize lsp_types::ServerCapabilities")?;

    let init_params = match connection.initialize(capabilities) {
        Ok(params) => params,
        Err(e) => {
            if e.channel_is_disconnected() {
                threads
                    .join()
                    .map_err(|_| "failed to wait on thread joining")?;
            }
            return Err(e.into());
        }
    };

    event_loop(connection, init_params)?;

    threads
        .join()
        .map_err(|_| "failed to wait on thread joining")?;

    lsp_log!("shutting down language server");
    Ok(())
}

fn event_loop(connection: Connection, params: serde_json::Value) -> Result<(), LspError> {
    lsp_log!("starting event loop");
    todo!()
}
