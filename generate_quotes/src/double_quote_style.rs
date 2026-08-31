pub(super) fn double_quote_style() -> crate::quote_style::QuoteStyle {
    crate::build_quote_style::build_quote_style(
        crate::quote_panic_id::QuotePanicId::from(constants_str::VALUE_0391AC99),
        crate::quote_prefix::QuotePrefix::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        crate::quote_char::QuoteChar::from('"'),
    )
}
