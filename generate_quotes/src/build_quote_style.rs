pub(crate) const fn build_quote_style(
    panic_id: crate::quote_panic_id::QuotePanicId,
    prefix: crate::quote_prefix::QuotePrefix,
    quote_ch: crate::quote_char::QuoteChar,
) -> crate::quote_style::QuoteStyle {
    crate::quote_style::QuoteStyle {
        panic_id,
        prefix,
        quote_ch,
    }
}
