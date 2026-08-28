pub(super) fn binary_single_quote_style() -> super::QuoteStyle {
    super::build_quote_style(
        super::QuotePanicId::from(constants_str::VALUE_8BCE26E7),
        super::QuotePrefix::from(constants_str::B),
        super::QuoteChar::from('\''),
    )
}
