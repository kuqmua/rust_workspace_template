pub(super) fn quote_str<DisplayValue>(
    quote_style: crate::quote_style::QuoteStyle,
    display_value: &DisplayValue,
) -> Result<
    crate::quoted_literal::QuotedLiteral,
    crate::quoted_literal::QuotedLiteralTryFromStringError,
>
where
    DisplayValue: std::fmt::Display + ?Sized,
{
    crate::quote_literal::quote_literal(
        quote_style.prefix(),
        quote_style.quote_character(),
        display_value,
    )
}
