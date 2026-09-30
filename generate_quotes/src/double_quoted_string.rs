pub fn double_quoted_string<DisplayValue>(
    display_value: &DisplayValue,
) -> Result<
    crate::quoted_literal::QuotedLiteral,
    crate::quoted_literal::QuotedLiteralTryFromStringError,
>
where
    DisplayValue: std::fmt::Display + ?Sized,
{
    crate::quote_str::quote_str(
        crate::double_quote_style::double_quote_style(),
        display_value,
    )
}
