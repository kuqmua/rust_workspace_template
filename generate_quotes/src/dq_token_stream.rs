#[must_use]
pub fn dq_token_stream<Dsp>(v: &Dsp) -> super::ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    super::quote_token_stream(super::double_quote_style(), v)
}
