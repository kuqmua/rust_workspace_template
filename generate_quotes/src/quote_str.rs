pub(super) fn quote_str<Dsp>(
    quote_style: crate::quote_style::QuoteStyle,
    dsp: &Dsp,
) -> crate::quoted_literal::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    let (_panic_id, prefix, quote_ch) = quote_style.into_parts();
    crate::quote_literal::quote_literal(prefix, quote_ch, dsp)
}
