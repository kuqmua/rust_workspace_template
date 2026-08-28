#[must_use]
pub fn dq_token_stream<Dsp>(v: &Dsp) -> crate::domain_types::ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::domain_types::quote_token_stream(crate::domain_types::double_quote_style(), v)
}
