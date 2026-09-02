#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]

pub fn generate_struct_identifier_with_number_elements_double_quoted_token_stream(
    identifier: &dyn naming::display_plus_to_tokens::DisplayPlusToTokens,
    struct_elements_length: crate::struct_elements_length::StructElementsLength,
) -> generate_quotes::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream {
    generate_quotes::dq_token_stream::dq_token_stream(&format!(
        "struct {identifier} with {} elements",
        struct_elements_length.get()
    ))
}
