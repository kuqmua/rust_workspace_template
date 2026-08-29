#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]

pub fn generate_de_double_quoted_token_stream(
    identifier: &dyn naming::display_plus_to_tokens::DisplayPlusToTokens,
    len: crate::de_len::DeLen,
) -> (
    generate_quotes::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream,
    generate_quotes::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream,
    generate_quotes::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream,
) {
    let struct_pg_type_identifier_where_tokens_double_quoted_token_stream =
        crate::generate_struct_identifier_double_quoted_token_stream::generate_struct_identifier_double_quoted_token_stream(identifier);
    let struct_pg_type_identifier_where_tokens_with_number_els_double_quoted_token_stream =
        crate::generate_struct_identifier_with_number_els_double_quoted_token_stream::generate_struct_identifier_with_number_els_double_quoted_token_stream(
            identifier,
            crate::struct_els_len::StructElsLen::from(len.get()),
        );
    let pg_type_identifier_where_tokens_double_quoted_token_stream =
        generate_quotes::dq_token_stream::dq_token_stream(&identifier);
    (
        struct_pg_type_identifier_where_tokens_double_quoted_token_stream,
        struct_pg_type_identifier_where_tokens_with_number_els_double_quoted_token_stream,
        pg_type_identifier_where_tokens_double_quoted_token_stream,
    )
}
