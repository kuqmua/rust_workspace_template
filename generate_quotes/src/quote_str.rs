pub(super) fn quote_str<Dsp>(style: super::QuoteStyle, value: &Dsp) -> super::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    super::quote_literal(style.prefix, style.quote_ch, value)
}
