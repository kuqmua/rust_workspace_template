pub fn generate_struct_identifier_double_quoted_token_stream(
    v: &dyn std::fmt::Display,
) -> generate_quotes::domain_types::ProcMacro2QuotedLiteralTokenStream {
    generate_quotes::domain_types::dq_token_stream(&format!("struct {v}"))
}
