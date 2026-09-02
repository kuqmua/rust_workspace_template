#[must_use]
pub fn binary_single_quotes_token_stream<Dsp>(
    dsp: &Dsp,
) -> crate::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::quote_token_stream::quote_token_stream(
        crate::binary_single_quote_style::binary_single_quote_style(),
        dsp,
    )
}
