#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]

pub(super) fn generate_update_query_string(
    pg_table_name_ref: crate::pg_table_name_ref::PgTableNameRef<'_>,
    cols_or_els: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    primary_key_field_name: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    primary_key_selector: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    cols_to_return: crate::pg_table_sql_fragment_ref::PgTableSqlFragmentRef<'_>,
    update_selector_fmt: crate::update_selector_fmt::UpdateSelectorFmt,
) -> crate::pg_table_query_string::PgTableQueryString {
    let selector_len = match update_selector_fmt {
        crate::update_selector_fmt::UpdateSelectorFmt::Eq => 3usize,
        crate::update_selector_fmt::UpdateSelectorFmt::InList => 6usize,
    };
    let mut query = String::with_capacity(
        30usize
            .saturating_add(pg_table_name_ref.as_ref().len())
            .saturating_add(cols_or_els.as_ref().len())
            .saturating_add(primary_key_field_name.as_ref().len())
            .saturating_add(primary_key_selector.as_ref().len())
            .saturating_add(cols_to_return.as_ref().len())
            .saturating_add(selector_len),
    );
    query.push_str(constants_str::UPDATE_ALT);
    query.push_str(pg_table_name_ref.as_ref());
    query.push_str(constants_str::SET);
    query.push_str(cols_or_els.as_ref());
    query.push_str(constants_str::WHERE);
    query.push_str(primary_key_field_name.as_ref());
    match update_selector_fmt {
        crate::update_selector_fmt::UpdateSelectorFmt::Eq => {
            query.push_str(constants_str::TEXT_ALT);
        }
        crate::update_selector_fmt::UpdateSelectorFmt::InList => {
            query.push_str(constants_str::IN);
        }
    }
    query.push_str(primary_key_selector.as_ref());
    if matches!(
        update_selector_fmt,
        crate::update_selector_fmt::UpdateSelectorFmt::InList
    ) {
        query.push(')');
    }
    query.push_str(constants_str::RETURNING);
    query.push_str(cols_to_return.as_ref());
    crate::pg_table_query_string::PgTableQueryString::try_from(query)
        .unwrap_or_else(crate::pg_table_query_string::PgTableQueryString::from)
}
