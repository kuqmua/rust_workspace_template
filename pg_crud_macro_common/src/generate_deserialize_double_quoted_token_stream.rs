#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]

pub fn generate_deserialize_double_quoted_token_stream(
    identifier: &dyn naming::display_plus_to_tokens::DisplayPlusToTokens,
    deserialize_length: crate::deserialize_length::DeserializeLength,
) -> (
    generate_quotes::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream,
    generate_quotes::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream,
    generate_quotes::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream,
) {
    let struct_pg_type_identifier_where_tokens_double_quoted_token_stream =
        crate::generate_struct_identifier_double_quoted_token_stream::generate_struct_identifier_double_quoted_token_stream(identifier);
    let struct_pg_type_identifier_where_tokens_with_number_elements_double_quoted_token_stream =
        crate::generate_struct_identifier_with_number_elements_double_quoted_token_stream::generate_struct_identifier_with_number_elements_double_quoted_token_stream(
            identifier,
            crate::struct_elements_length::StructElementsLength::from(deserialize_length.get()),
        );
    let pg_type_identifier_where_tokens_double_quoted_token_stream =
        generate_quotes::double_quoted_token_stream::double_quoted_token_stream(&identifier);
    (
        struct_pg_type_identifier_where_tokens_double_quoted_token_stream,
        struct_pg_type_identifier_where_tokens_with_number_elements_double_quoted_token_stream,
        pg_type_identifier_where_tokens_double_quoted_token_stream,
    )
}
