pub(super) fn quote_str<Dsp>(
    style: crate::quote_style::QuoteStyle,
    value: &Dsp,
) -> crate::quoted_literal::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    let (_panic_id, prefix, quote_ch) = style.into_parts();
    crate::quote_literal::quote_literal(prefix, quote_ch, value)
}
