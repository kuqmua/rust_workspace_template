pub(super) fn quote_literal<DisplayValue>(
    quote_prefix: crate::quote_prefix::QuotePrefix,
    quote_char: crate::quote_char::QuoteChar,
    display_value: &DisplayValue,
) -> Result<
    crate::quoted_literal::QuotedLiteral,
    crate::quoted_literal::QuotedLiteralTryFromStringError,
>
where
    DisplayValue: std::fmt::Display + ?Sized,
{
    let prefix_text: &str = quote_prefix.into();
    let quote_character = char::from(quote_char);
    let mut output = String::with_capacity(prefix_text.len().saturating_add(2));
    output.push_str(prefix_text);
    output.push(quote_character);
    if std::fmt::Write::write_fmt(&mut output, format_args!("{display_value}")).is_err() {
        return crate::quoted_literal::QuotedLiteral::try_from(format!(
            "{prefix_text}{quote_character}{display_value}{quote_character}"
        ));
    }
    output.push(quote_character);
    crate::quoted_literal::QuotedLiteral::try_from(output)
}
