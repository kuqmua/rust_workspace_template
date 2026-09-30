pub fn single_quotes_str<DisplayValue>(
    display_value: &DisplayValue,
) -> Result<
    crate::quoted_literal::QuotedLiteral,
    crate::quoted_literal::QuotedLiteralTryFromStringError,
>
where
    DisplayValue: std::fmt::Display + ?Sized,
{
    crate::quote_str::quote_str(
        crate::single_quote_style::single_quote_style(),
        display_value,
    )
}
