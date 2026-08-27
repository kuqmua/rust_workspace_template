#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved implementation"
)]
use super::*;

#[must_use]
pub fn generate_column_eqs_case_accumulator_else_column_end_comma_um_query_part(
    column: PgTableSqlFragmentRef<'_>,
    accumulator: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryPartFragment {
    let mut query_part = String::with_capacity(
        column
            .as_ref()
            .len()
            .saturating_mul(2)
            .saturating_add(accumulator.as_ref().len())
            .saturating_add(19),
    );
    if std::fmt::Write::write_fmt(
        &mut query_part,
        format_args!("{column} = case {accumulator}else {column} end,"),
    )
    .is_err()
    {
        return PgTableQueryPartFragment::try_from(String::default())
            .unwrap_or_else(PgTableQueryPartFragment::from);
    }
    PgTableQueryPartFragment::try_from(query_part).unwrap_or_else(PgTableQueryPartFragment::from)
}
