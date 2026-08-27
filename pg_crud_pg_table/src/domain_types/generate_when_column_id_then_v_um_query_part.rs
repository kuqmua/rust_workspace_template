#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

#[must_use]
pub fn generate_when_column_id_then_v_um_query_part(
    column: PgTableSqlFragmentRef<'_>,
    id: PgTableSqlFragmentRef<'_>,
    value: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryPartFragment {
    let mut query_part = String::with_capacity(
        column
            .as_ref()
            .len()
            .saturating_add(id.as_ref().len())
            .saturating_add(value.as_ref().len())
            .saturating_add(15),
    );
    if std::fmt::Write::write_fmt(
        &mut query_part,
        format_args!("when {column} = {id} then {value} "),
    )
    .is_err()
    {
        return PgTableQueryPartFragment::try_from(String::default())
            .unwrap_or_else(PgTableQueryPartFragment::from);
    }
    PgTableQueryPartFragment::try_from(query_part).unwrap_or_else(PgTableQueryPartFragment::from)
}
