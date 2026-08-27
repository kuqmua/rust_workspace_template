#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]
use super::super::*;

pub fn generate_struct_identifier_with_number_els_double_quoted_token_stream(
    identifier: &dyn naming::domain_types::DisplayPlusToTokens,
    len: StructElsLen,
) -> generate_quotes::domain_types::ProcMacro2QuotedLiteralTokenStream {
    generate_quotes::domain_types::dq_token_stream(&format!(
        "struct {identifier} with {} els",
        len.get()
    ))
}
