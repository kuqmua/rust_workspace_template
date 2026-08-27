#[must_use]
pub fn single_quotes_str<Dsp>(v: &Dsp) -> super::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    super::quote_str(super::single_quote_style(), v)
}
