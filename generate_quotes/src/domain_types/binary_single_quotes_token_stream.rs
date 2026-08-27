#[must_use]
pub fn binary_single_quotes_token_stream<Dsp>(v: &Dsp) -> super::ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    super::quote_token_stream(super::binary_single_quote_style(), v)
}
