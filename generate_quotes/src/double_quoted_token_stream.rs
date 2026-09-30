#[must_use]
pub fn double_quoted_token_stream<DisplayValue>(
    display_value: &DisplayValue,
) -> crate::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream
where
    DisplayValue: std::fmt::Display + ?Sized,
{
    crate::quote_token_stream::quote_token_stream(
        crate::double_quote_style::double_quote_style(),
        display_value,
    )
}
