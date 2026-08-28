#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

pub(super) fn generate_update_query_string(
    table: PgTableNameRef<'_>,
    cols_or_els: PgTableSqlFragmentRef<'_>,
    primary_key_field_name: PgTableSqlFragmentRef<'_>,
    primary_key_selector: PgTableSqlFragmentRef<'_>,
    cols_to_return: PgTableSqlFragmentRef<'_>,
    update_selector_fmt: UpdateSelectorFmt,
) -> PgTableQueryString {
    let selector_len = match update_selector_fmt {
        UpdateSelectorFmt::Eq => 3usize,
        UpdateSelectorFmt::InList => 6usize,
    };
    let mut query = String::with_capacity(
        30usize
            .saturating_add(table.as_ref().len())
            .saturating_add(cols_or_els.as_ref().len())
            .saturating_add(primary_key_field_name.as_ref().len())
            .saturating_add(primary_key_selector.as_ref().len())
            .saturating_add(cols_to_return.as_ref().len())
            .saturating_add(selector_len),
    );
    query.push_str(constants_str::UPDATE_ALT);
    query.push_str(table.as_ref());
    query.push_str(constants_str::SET);
    query.push_str(cols_or_els.as_ref());
    query.push_str(constants_str::WHERE);
    query.push_str(primary_key_field_name.as_ref());
    match update_selector_fmt {
        UpdateSelectorFmt::Eq => query.push_str(constants_str::TEXT_ALT),
        UpdateSelectorFmt::InList => query.push_str(constants_str::IN),
    }
    query.push_str(primary_key_selector.as_ref());
    if matches!(update_selector_fmt, UpdateSelectorFmt::InList) {
        query.push(')');
    }
    query.push_str(constants_str::RETURNING);
    query.push_str(cols_to_return.as_ref());
    PgTableQueryString::try_from(query).unwrap_or_else(PgTableQueryString::from)
}
