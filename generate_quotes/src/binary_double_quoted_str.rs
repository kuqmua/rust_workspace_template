pub fn binary_double_quoted_str<Dsp>(
    dsp: &Dsp,
) -> Result<
    crate::quoted_literal::QuotedLiteral,
    crate::quoted_literal::QuotedLiteralTryFromStringError,
>
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::quote_str::quote_str(
        crate::binary_double_quote_style::binary_double_quote_style(),
        dsp,
    )
}
