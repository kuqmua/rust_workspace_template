pub(super) fn single_quote_style() -> crate::domain_types::QuoteStyle {
    crate::domain_types::build_quote_style(
        crate::domain_types::QuotePanicId::from(constants_str::EC1E77D5),
        crate::domain_types::QuotePrefix::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        crate::domain_types::QuoteChar::from('\''),
    )
}
