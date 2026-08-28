pub(super) fn single_quote_style() -> super::QuoteStyle {
    super::build_quote_style(
        super::QuotePanicId::from(constants_str::EC1E77D5),
        super::QuotePrefix::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        super::QuoteChar::from('\''),
    )
}
