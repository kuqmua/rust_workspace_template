pub(super) fn quote_literal<Dsp>(
    prefix: crate::quote_prefix::QuotePrefix,
    quote_ch: crate::quote_char::QuoteChar,
    v: &Dsp,
) -> crate::quoted_literal::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    let prefix_text: &str = prefix.into();
    let quote_character = char::from(quote_ch);
    let mut out = String::with_capacity(prefix_text.len().saturating_add(2));
    out.push_str(prefix_text);
    out.push(quote_character);
    if std::fmt::Write::write_fmt(&mut out, format_args!("{v}")).is_err() {
        return crate::quoted_literal::QuotedLiteral::try_from(format!(
            "{prefix_text}{quote_character}{v}{quote_character}"
        ))
        .unwrap_or_else(crate::quoted_literal::QuotedLiteral::from);
    }
    out.push(quote_character);
    crate::quoted_literal::QuotedLiteral::try_from(out)
        .unwrap_or_else(crate::quoted_literal::QuotedLiteral::from)
}
