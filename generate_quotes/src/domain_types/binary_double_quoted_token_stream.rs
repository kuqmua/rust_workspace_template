#[must_use]
pub fn binary_double_quoted_token_stream<Dsp>(v: &Dsp) -> super::ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    super::quote_token_stream(super::binary_double_quote_style(), v)
}
