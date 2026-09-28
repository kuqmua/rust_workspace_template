pub(super) fn quote_str<Dsp>(
    quote_style: crate::quote_style::QuoteStyle,
    dsp: &Dsp,
) -> Result<
    crate::quoted_literal::QuotedLiteral,
    crate::quoted_literal::QuotedLiteralTryFromStringError,
>
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::quote_literal::quote_literal(quote_style.prefix(), quote_style.quote_ch(), dsp)
}
