#[must_use]
pub fn binary_double_quoted_str<Dsp>(dsp: &Dsp) -> crate::quoted_literal::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::quote_str::quote_str(
        crate::binary_double_quote_style::binary_double_quote_style(),
        dsp,
    )
}
