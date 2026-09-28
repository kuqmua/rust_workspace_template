pub fn double_quoted_string<Dsp>(
    dsp: &Dsp,
) -> Result<
    crate::quoted_literal::QuotedLiteral,
    crate::quoted_literal::QuotedLiteralTryFromStringError,
>
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::quote_str::quote_str(crate::double_quote_style::double_quote_style(), dsp)
}
