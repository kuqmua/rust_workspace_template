#[must_use]
pub fn binary_double_quoted_str<Dsp>(v: &Dsp) -> crate::domain_types::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::domain_types::quote_str(crate::domain_types::binary_double_quote_style(), v)
}
