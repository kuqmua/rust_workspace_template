#[must_use]
pub fn binary_single_quotes_str<Dsp>(v: &Dsp) -> super::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    super::quote_str(super::binary_single_quote_style(), v)
}
