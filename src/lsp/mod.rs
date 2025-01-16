mod error;
mod handlers;

use error::LspError;
use lsp_server::{Connection, ExtractError, Message, Notification, Request, RequestId};
use lsp_types::{
    notification::DidOpenTextDocument, request::HoverRequest, InitializeParams, ServerCapabilities,
};

use crate::{lexer::Lexer, types::Token};

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
        hover_provider: Some(lsp_types::HoverProviderCapability::Simple(true)),
        // TODO: this should be incremental
        text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(
            lsp_types::TextDocumentSyncKind::FULL,
        )),
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
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    lsp_log!("starting event loop");
    let mut tokens: Vec<Token> = vec![];
    let mut errors: Vec<super::error::Error> = vec![];
    for msg in &connection.receiver {
        eprintln!("got msg: {msg:?}");
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                eprintln!("got request: {req:?}");
                match req.method.as_str() {
                    "textDocument/hover" => {
                        match cast::<HoverRequest>(req) {
                            Ok((id, params)) => {
                                if let Err(e) =
                                    handlers::hover::handle(&connection, &tokens, id, params)
                                {
                                    eprintln!("[sqleibniz]: err: {}", e);
                                }
                                continue;
                            }
                            Err(err @ _) => panic!("{err:?}"),
                        };
                    }
                    _ => lsp_log!("unsupported method"),
                }
                // ...
            }
            Message::Response(resp) => {
                eprintln!("got response: {resp:?}");
            }
            Message::Notification(not) => match not.method.as_str() {
                "textDocument/didOpen" => {
                    match cast_noti::<DidOpenTextDocument>(not) {
                        Ok(params) => {
                            let text = &(params.text_document.text.into_bytes());
                            let formatted_path =
                                params.text_document.uri.to_string().replace("file://", "");
                            let mut l = Lexer::new(text, &formatted_path);
                            tokens = l.run();
                            errors.append(&mut l.errors);
                        }
                        Err(err @ _) => panic!("{err:?}"),
                    };
                }
                _ => lsp_log!("unsupported method"),
            },
        }
    }
    Ok(())
}

fn cast<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}

fn cast_noti<N>(not: Notification) -> Result<N::Params, ExtractError<Notification>>
where
    N: lsp_types::notification::Notification,
    N::Params: serde::de::DeserializeOwned,
{
    not.extract(N::METHOD)
}
