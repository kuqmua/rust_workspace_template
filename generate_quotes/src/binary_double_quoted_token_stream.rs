#[must_use]
pub fn binary_double_quoted_token_stream<Dsp>(
    v: &Dsp,
) -> crate::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::quote_token_stream::quote_token_stream(
        crate::binary_double_quote_style::binary_double_quote_style(),
        v,
    )
}
