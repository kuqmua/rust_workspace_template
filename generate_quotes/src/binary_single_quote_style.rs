pub(super) fn binary_single_quote_style() -> crate::domain_types::QuoteStyle {
    crate::domain_types::build_quote_style(
        crate::domain_types::QuotePanicId::from(constants_str::VALUE_8BCE26E7),
        crate::domain_types::QuotePrefix::from(constants_str::B),
        crate::domain_types::QuoteChar::from('\''),
    )
}
