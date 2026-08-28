pub(super) fn quote_str<Dsp>(
    style: crate::domain_types::QuoteStyle,
    value: &Dsp,
) -> crate::domain_types::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::domain_types::quote_literal(style.prefix, style.quote_ch, value)
}
