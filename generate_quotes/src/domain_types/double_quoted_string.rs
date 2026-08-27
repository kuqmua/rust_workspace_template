#[must_use]
pub fn double_quoted_string<Dsp>(v: &Dsp) -> super::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    super::quote_str(super::double_quote_style(), v)
}
