#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]
use crate::domain_types::*;

pub fn generate_de_double_quoted_token_stream(
    identifier: &dyn naming::domain_types::DisplayPlusToTokens,
    len: DeLen,
) -> (
    generate_quotes::domain_types::ProcMacro2QuotedLiteralTokenStream,
    generate_quotes::domain_types::ProcMacro2QuotedLiteralTokenStream,
    generate_quotes::domain_types::ProcMacro2QuotedLiteralTokenStream,
) {
    let struct_pg_type_identifier_where_tokens_double_quoted_token_stream =
        generate_struct_identifier_double_quoted_token_stream(identifier);
    let struct_pg_type_identifier_where_tokens_with_number_els_double_quoted_token_stream =
        generate_struct_identifier_with_number_els_double_quoted_token_stream(
            identifier,
            StructElsLen::from(len.get()),
        );
    let pg_type_identifier_where_tokens_double_quoted_token_stream =
        generate_quotes::domain_types::dq_token_stream(&identifier);
    (
        struct_pg_type_identifier_where_tokens_double_quoted_token_stream,
        struct_pg_type_identifier_where_tokens_with_number_els_double_quoted_token_stream,
        pg_type_identifier_where_tokens_double_quoted_token_stream,
    )
}
