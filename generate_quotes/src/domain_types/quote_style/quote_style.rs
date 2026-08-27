pub(crate) const fn quote_style(
    panic_id: super::super::QuotePanicId,
    prefix: super::super::QuotePrefix,
    quote_ch: super::super::QuoteChar,
) -> super::QuoteStyle {
    super::QuoteStyle {
        panic_id,
        prefix,
        quote_ch,
    }
}
