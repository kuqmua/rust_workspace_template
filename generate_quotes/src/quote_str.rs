pub(super) fn quote_str<Dsp>(
    style: crate::quote_style::QuoteStyle,
    value: &Dsp,
) -> crate::quoted_literal::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::quote_literal::quote_literal(style.prefix, style.quote_ch, value)
}
