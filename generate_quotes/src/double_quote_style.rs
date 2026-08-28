pub(super) fn double_quote_style() -> super::QuoteStyle {
    super::build_quote_style(
        super::QuotePanicId::from(constants_str::VALUE_0391AC99),
        super::QuotePrefix::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        super::QuoteChar::from('"'),
    )
}
