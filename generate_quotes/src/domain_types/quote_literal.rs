pub(super) fn quote_literal<Dsp>(
    prefix: super::QuotePrefix,
    quote_ch: super::QuoteChar,
    v: &Dsp,
) -> super::QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    let mut out = String::with_capacity(prefix.0.len().saturating_add(2));
    out.push_str(prefix.0);
    out.push(quote_ch.0);
    if std::fmt::Write::write_fmt(&mut out, format_args!("{v}")).is_err() {
        return super::QuotedLiteral::try_from(format!(
            "{}{}{v}{}",
            prefix.0, quote_ch.0, quote_ch.0
        ))
        .unwrap_or_else(super::QuotedLiteral::from);
    }
    out.push(quote_ch.0);
    super::QuotedLiteral::try_from(out).unwrap_or_else(super::QuotedLiteral::from)
}
