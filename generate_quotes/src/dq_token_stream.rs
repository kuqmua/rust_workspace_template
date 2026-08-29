#[must_use]
pub fn dq_token_stream<Dsp>(
    v: &Dsp,
) -> crate::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::quote_token_stream::quote_token_stream(
        crate::double_quote_style::double_quote_style(),
        v,
    )
}
