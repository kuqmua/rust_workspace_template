#[must_use]
pub fn binary_single_quotes_str<Dsp>(v: &Dsp) -> crate::quoted_literal::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::quote_str::quote_str(
        crate::binary_single_quote_style::binary_single_quote_style(),
        v,
    )
}
