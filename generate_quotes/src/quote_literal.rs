pub(super) fn quote_literal<Dsp>(
    quote_prefix: crate::quote_prefix::QuotePrefix,
    quote_char: crate::quote_char::QuoteChar,
    dsp: &Dsp,
) -> crate::quoted_literal::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    let prefix_text: &str = quote_prefix.into();
    let quote_character = char::from(quote_char);
    let mut out = String::with_capacity(prefix_text.len().saturating_add(2));
    out.push_str(prefix_text);
    out.push(quote_character);
    if std::fmt::Write::write_fmt(&mut out, format_args!("{dsp}")).is_err() {
        return crate::quoted_literal::QuotedLiteral::try_from(format!(
            "{prefix_text}{quote_character}{dsp}{quote_character}"
        ))
        .unwrap_or_else(crate::quoted_literal::QuotedLiteral::from);
    }
    out.push(quote_character);
    crate::quoted_literal::QuotedLiteral::try_from(out)
        .unwrap_or_else(crate::quoted_literal::QuotedLiteral::from)
}
