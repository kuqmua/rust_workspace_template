pub(crate) fn build_quote_style(
    quote_panic_id: crate::quote_panic_id::QuotePanicId,
    quote_prefix: crate::quote_prefix::QuotePrefix,
    quote_char: crate::quote_char::QuoteChar,
) -> crate::quote_style::QuoteStyle {
    crate::quote_style::QuoteStyle::from((quote_panic_id, quote_prefix, quote_char))
}
