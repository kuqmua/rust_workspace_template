pub fn generate_struct_identifier_double_quoted_token_stream(
    v: &dyn std::fmt::Display,
) -> generate_quotes::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream {
    generate_quotes::dq_token_stream::dq_token_stream(&format!("struct {v}"))
}
