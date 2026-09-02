#[must_use]
pub fn single_quotes_str<Dsp>(dsp: &Dsp) -> crate::quoted_literal::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::quote_str::quote_str(crate::single_quote_style::single_quote_style(), dsp)
}
