pub(super) fn binary_single_quote_style() -> crate::quote_style::QuoteStyle {
    crate::build_quote_style::build_quote_style(
        crate::quote_panic_id::QuotePanicId::from(constants_str::VALUE_8BCE26E7),
        crate::quote_prefix::QuotePrefix::from(constants_str::B),
        crate::quote_char::QuoteChar::from('\''),
    )
}
