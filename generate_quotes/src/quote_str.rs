pub(super) fn quote_str<Dsp>(
    quote_style: crate::quote_style::QuoteStyle,
    dsp: &Dsp,
) -> crate::quoted_literal::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::quote_literal::quote_literal(quote_style.prefix(), quote_style.quote_ch(), dsp)
}
