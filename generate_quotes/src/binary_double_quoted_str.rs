pub fn binary_double_quoted_str<DisplayValue>(
    display_value: &DisplayValue,
) -> Result<
    crate::quoted_literal::QuotedLiteral,
    crate::quoted_literal::QuotedLiteralTryFromStringError,
>
where
    DisplayValue: std::fmt::Display + ?Sized,
{
    crate::quote_str::quote_str(
        crate::binary_double_quote_style::binary_double_quote_style(),
        display_value,
    )
}
