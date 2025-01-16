use lsp_server::{Connection, Message, RequestId, Response};
use lsp_types::{Diagnostic, DiagnosticSeverity, DocumentDiagnosticParams, Position, Range};

use crate::{error::Error, lsp::error::LspError};

impl From<Error> for Diagnostic {
    fn from(value: Error) -> Self {
        Self {
            range: Range::new(
                Position {
                    line: value.line as u32,
                    character: value.start as u32,
                },
                Position {
                    line: value.line as u32,
                    character: value.end as u32,
                },
            ),
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(lsp_types::NumberOrString::String(
                value.rule.name().to_string(),
            )),
            code_description: None,
            source: Some("sqleibniz".into()),
            message: format!("{}: {}", value.msg, value.note),
            related_information: None,
            tags: None,
            data: None,
        }
    }
}

pub fn handle(
    connection: &Connection,
    errors: Vec<Error>,
    id: RequestId,
    _: DocumentDiagnosticParams,
) -> Result<(), LspError> {
    eprintln!("got diagnostic request #{id}");
    let diagnostics = lsp_types::FullDocumentDiagnosticReport {
        result_id: None,
        items: errors.into_iter().map(Error::into).collect(),
    };
    let result = serde_json::to_value(&diagnostics).unwrap();
    let resp = Response {
        id,
        result: Some(result),
        error: None,
    };
    connection
        .sender
        .send(Message::Response(resp))
        .map_err(|_| "failed to send diagnostics")?;
    Ok(())
}
