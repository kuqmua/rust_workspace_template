#[must_use]
pub fn double_quoted_string<Dsp>(v: &Dsp) -> crate::quoted_literal::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::quote_str::quote_str(crate::double_quote_style::double_quote_style(), v)
}
