#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySortOrder {
    Ascending,
    Descending,
}
impl QuerySortOrder {
    fn sql(self) -> SqlSortOrderText {
        SqlSortOrderText::from(match self {
            Self::Ascending => str_constants::SORT_ASC,
            Self::Descending => str_constants::SORT_DESC,
        })
    }
}

#[derive(
    optml::Optml, Clone, Copy, Debug, Eq, PartialEq, newtype::AsRefInner, newtype::FromInner,
)]
struct SqlSortOrderText(&'static str);

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::IntoInnerFrom, newtype::FromInner)]
pub struct ReadQueryPlan(crate::QueryPartFragment);

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("read query plan exceeds the query fragment limit")]
pub struct ReadQueryPlanError;

pub fn build_stable_read_query_plan(
    base: crate::QueryPartFragment,
    sort_column: &crate::SqlIdentifier,
    tie_break_column: &crate::SqlIdentifier,
    order: QuerySortOrder,
    limit_bind: crate::StdReadQueryBindIndex,
    offset_bind: crate::StdReadQueryBindIndex,
) -> Result<ReadQueryPlan, ReadQueryPlanError> {
    let mut query = base.into_inner();
    let order_sql = order.sql();
    let tie_break_len = if sort_column == tie_break_column {
        0usize
    } else {
        str_constants::TEXT_ALT_6
            .len()
            .saturating_add(tie_break_column.as_ref().len())
            .saturating_add(1usize)
            .saturating_add(order_sql.as_ref().len())
    };
    query.reserve(
        str_constants::READ_ORDER_BY
            .len()
            .saturating_add(sort_column.as_ref().len())
            .saturating_add(1usize)
            .saturating_add(order_sql.as_ref().len())
            .saturating_add(tie_break_len)
            .saturating_add(str_constants::LIMIT_DOLLAR.len())
            .saturating_add(10usize)
            .saturating_add(str_constants::OFFSET_DOLLAR.len())
            .saturating_add(10usize),
    );
    query.push_str(str_constants::READ_ORDER_BY);
    query.push_str(sort_column.as_ref());
    query.push(' ');
    query.push_str(order_sql.as_ref());
    if sort_column != tie_break_column {
        query.push_str(str_constants::TEXT_ALT_6);
        query.push_str(tie_break_column.as_ref());
        query.push(' ');
        query.push_str(order_sql.as_ref());
    }
    query.push_str(str_constants::LIMIT_DOLLAR);
    let mut query_fragment =
        crate::QueryPartFragment::try_from(query).map_err(|_error| ReadQueryPlanError)?;
    query_fragment.append_read_bind_index(limit_bind)?;
    std::fmt::Write::write_str(&mut query_fragment, str_constants::OFFSET_DOLLAR)
        .map_err(|_error| ReadQueryPlanError)?;
    query_fragment.append_read_bind_index(offset_bind)?;
    Ok(ReadQueryPlan::from(query_fragment))
}

#[cfg(test)]
mod tests {
    fn identifier(value: &str) -> crate::SqlIdentifier {
        crate::SqlIdentifier::try_from(value.to_owned())
            .expect("cd7c83ed identifier invariant must hold")
    }

    #[test]
    fn stable_plan_appends_tie_break_limit_and_offset() {
        let plan = super::build_stable_read_query_plan(
            crate::QueryPartFragment::try_from(String::from(str_constants::TEST_READ_QUERY_BASE))
                .expect(
                    "ef7cd3e2 stable_plan_appends_tie_break_limit_and_offset invariant must hold",
                ),
            &identifier(str_constants::CREATED_AT),
            &identifier(str_constants::SQL_NAMES_ID),
            super::QuerySortOrder::Descending,
            std::num::NonZeroU32::new(1u32)
                .expect(
                    "2c810064 stable_plan_appends_tie_break_limit_and_offset invariant must hold",
                )
                .into(),
            std::num::NonZeroU32::new(2u32)
                .expect(
                    "aa77f541 stable_plan_appends_tie_break_limit_and_offset invariant must hold",
                )
                .into(),
        )
        .expect("377c56d0 stable_plan_appends_tie_break_limit_and_offset invariant must hold");
        let fragment = crate::QueryPartFragment::from(plan);
        assert_eq!(fragment.into_inner(), str_constants::TEST_STABLE_READ_QUERY);
    }
}
