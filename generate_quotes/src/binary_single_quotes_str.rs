#[must_use]
pub fn binary_single_quotes_str<Dsp>(v: &Dsp) -> crate::domain_types::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::domain_types::quote_str(crate::domain_types::binary_single_quote_style(), v)
}
