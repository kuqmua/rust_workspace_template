pub(super) fn quote_literal<Dsp>(
    prefix: crate::domain_types::QuotePrefix,
    quote_ch: crate::domain_types::QuoteChar,
    v: &Dsp,
) -> crate::domain_types::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    let mut out = String::with_capacity(prefix.0.len().saturating_add(2));
    out.push_str(prefix.0);
    out.push(quote_ch.0);
    if std::fmt::Write::write_fmt(&mut out, format_args!("{v}")).is_err() {
        return crate::domain_types::QuotedLiteral::try_from(format!(
            "{}{}{v}{}",
            prefix.0, quote_ch.0, quote_ch.0
        ))
        .unwrap_or_else(crate::domain_types::QuotedLiteral::from);
    }
    out.push(quote_ch.0);
    crate::domain_types::QuotedLiteral::try_from(out)
        .unwrap_or_else(crate::domain_types::QuotedLiteral::from)
}
