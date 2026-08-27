#[must_use]
pub fn binary_double_quoted_str<Dsp>(v: &Dsp) -> super::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    super::quote_str(super::binary_double_quote_style(), v)
}
