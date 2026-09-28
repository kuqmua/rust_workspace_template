pub fn binary_single_quotes_str<Dsp>(
    dsp: &Dsp,
) -> Result<
    crate::quoted_literal::QuotedLiteral,
    crate::quoted_literal::QuotedLiteralTryFromStringError,
>
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::quote_str::quote_str(
        crate::binary_single_quote_style::binary_single_quote_style(),
        dsp,
    )
}
