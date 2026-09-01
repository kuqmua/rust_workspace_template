pub fn build_stable_read_query_plan(
    base: crate::query_part_fragment::QueryPartFragment,
    sort_column: &crate::sql_identifier::SqlIdentifier,
    tie_break_column: &crate::sql_identifier::SqlIdentifier,
    order: crate::query_sort_order::QuerySortOrder,
    limit_bind: crate::read_query_bind_index_non_zero_u32::ReadQueryBindIndexNonZeroU32,
    offset_bind: crate::read_query_bind_index_non_zero_u32::ReadQueryBindIndexNonZeroU32,
) -> Result<crate::read_query_plan::ReadQueryPlan, crate::read_query_plan_error::ReadQueryPlanError>
{
    let mut query = base.into_inner();
    let order_sql = order.sql();
    let tie_break_len = if sort_column == tie_break_column {
        constants_usize::ZERO
    } else {
        constants_str::TEXT_ALT_6
            .len()
            .saturating_add(tie_break_column.as_ref().len())
            .saturating_add(constants_usize::ONE)
            .saturating_add(order_sql.as_ref().len())
    };
    query.reserve(
        constants_str::READ_ORDER_BY
            .len()
            .saturating_add(sort_column.as_ref().len())
            .saturating_add(constants_usize::ONE)
            .saturating_add(order_sql.as_ref().len())
            .saturating_add(tie_break_len)
            .saturating_add(constants_str::LIMIT_DOLLAR.len())
            .saturating_add(10usize)
            .saturating_add(constants_str::OFFSET_DOLLAR.len())
            .saturating_add(10usize),
    );
    query.push_str(constants_str::READ_ORDER_BY);
    query.push_str(sort_column.as_ref());
    query.push(' ');
    query.push_str(order_sql.as_ref());
    if sort_column != tie_break_column {
        query.push_str(constants_str::TEXT_ALT_6);
        query.push_str(tie_break_column.as_ref());
        query.push(' ');
        query.push_str(order_sql.as_ref());
    }
    query.push_str(constants_str::LIMIT_DOLLAR);
    let mut query_fragment = crate::query_part_fragment::QueryPartFragment::try_from(query)
        .map_err(|_error| crate::read_query_plan_error::ReadQueryPlanError::TooManyFragments)?;
    query_fragment.append_read_bind_index(limit_bind)?;
    std::fmt::Write::write_str(&mut query_fragment, constants_str::OFFSET_DOLLAR)
        .map_err(|_error| crate::read_query_plan_error::ReadQueryPlanError::TooManyFragments)?;
    query_fragment.append_read_bind_index(offset_bind)?;
    Ok(crate::read_query_plan::ReadQueryPlan::from(query_fragment))
}

#[cfg(test)]
mod tests {
    fn stable_read_identifier(value: &str) -> crate::sql_identifier::SqlIdentifier {
        crate::sql_identifier::SqlIdentifier::try_from(value.to_owned())
            .expect(constants_str::DIAGNOSTIC_CD7C83ED)
    }

    #[test]
    fn test_stable_plan_appends_tie_break_limit_and_offset() {
        let plan = crate::build_stable_read_query_plan::build_stable_read_query_plan(
            crate::query_part_fragment::QueryPartFragment::try_from(String::from(
                constants_str::TEST_READ_QUERY_BASE,
            ))
            .expect(constants_str::DIAGNOSTIC_EF7CD3E2),
            &stable_read_identifier(constants_str::CREATED_AT),
            &stable_read_identifier(constants_str::SQL_NAMES_ID),
            crate::query_sort_order::QuerySortOrder::Descending,
            std::num::NonZeroU32::new(1u32)
                .expect(constants_str::DIAGNOSTIC_2C810064)
                .into(),
            std::num::NonZeroU32::new(2u32)
                .expect(constants_str::DIAGNOSTIC_AA77F541)
                .into(),
        )
        .expect(constants_str::DIAGNOSTIC_377C56D0);
        let fragment = crate::query_part_fragment::QueryPartFragment::from(plan);
        assert_eq!(fragment.into_inner(), constants_str::TEST_STABLE_READ_QUERY);
    }
}
