#[must_use]
pub fn binary_double_quoted_token_stream<Dsp>(
    v: &Dsp,
) -> crate::domain_types::ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::domain_types::quote_token_stream(crate::domain_types::binary_double_quote_style(), v)
}
