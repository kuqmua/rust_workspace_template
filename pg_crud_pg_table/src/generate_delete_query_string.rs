#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

pub(super) fn generate_delete_query_string(
    table: PgTableNameRef<'_>,
    primary_key_field_name: PgTableSqlFragmentRef<'_>,
    where_string: Option<PgTableSqlFragmentRef<'_>>,
) -> PgTableQueryString {
    let where_len = where_string.map_or_else(
        || 12usize.saturating_add(primary_key_field_name.as_ref().len()),
        |v| v.as_ref().len(),
    );
    let mut query = String::with_capacity(
        24usize
            .saturating_add(table.as_ref().len())
            .saturating_add(where_len)
            .saturating_add(primary_key_field_name.as_ref().len()),
    );
    query.push_str(constants_str::DELETE_FROM);
    query.push_str(table.as_ref());
    query.push(' ');
    if let Some(v) = where_string {
        query.push_str(v.as_ref());
    } else {
        query.push_str(constants_str::WHERE_ALT);
        query.push_str(primary_key_field_name.as_ref());
        query.push_str(constants_str::DOLLAR_1);
    }
    query.push_str(constants_str::RETURNING);
    query.push_str(primary_key_field_name.as_ref());
    PgTableQueryString::try_from(query).unwrap_or_else(PgTableQueryString::from)
}
