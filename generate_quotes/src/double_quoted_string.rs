#[must_use]
pub fn double_quoted_string<Dsp>(v: &Dsp) -> crate::domain_types::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::domain_types::quote_str(crate::domain_types::double_quote_style(), v)
}
