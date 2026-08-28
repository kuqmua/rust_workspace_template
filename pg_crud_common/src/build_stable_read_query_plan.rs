pub fn build_stable_read_query_plan(
    base: crate::domain_types::QueryPartFragment,
    sort_column: &crate::domain_types::SqlIdentifier,
    tie_break_column: &crate::domain_types::SqlIdentifier,
    order: crate::domain_types::QuerySortOrder,
    limit_bind: crate::domain_types::ReadQueryBindIndexNonZeroU32,
    offset_bind: crate::domain_types::ReadQueryBindIndexNonZeroU32,
) -> Result<crate::domain_types::ReadQueryPlan, crate::domain_types::ReadQueryPlanError> {
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
    let mut query_fragment = crate::domain_types::QueryPartFragment::try_from(query)
        .map_err(|_error| crate::domain_types::ReadQueryPlanError)?;
    query_fragment.append_read_bind_index(limit_bind)?;
    std::fmt::Write::write_str(&mut query_fragment, constants_str::OFFSET_DOLLAR)
        .map_err(|_error| crate::domain_types::ReadQueryPlanError)?;
    query_fragment.append_read_bind_index(offset_bind)?;
    Ok(crate::domain_types::ReadQueryPlan::from(query_fragment))
}

#[cfg(test)]
mod tests {
    fn stable_read_identifier(value: &str) -> crate::domain_types::SqlIdentifier {
        crate::domain_types::SqlIdentifier::try_from(value.to_owned())
            .expect("cd7c83ed identifier invariant must hold")
    }

    #[test]
    fn stable_plan_appends_tie_break_limit_and_offset() {
        let plan = super::build_stable_read_query_plan(
            crate::domain_types::QueryPartFragment::try_from(String::from(
                constants_str::TEST_READ_QUERY_BASE,
            ))
            .expect("ef7cd3e2 stable_plan_appends_tie_break_limit_and_offset invariant must hold"),
            &stable_read_identifier(constants_str::CREATED_AT),
            &stable_read_identifier(constants_str::SQL_NAMES_ID),
            crate::domain_types::QuerySortOrder::Descending,
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
        let fragment = crate::domain_types::QueryPartFragment::from(plan);
        assert_eq!(fragment.into_inner(), constants_str::TEST_STABLE_READ_QUERY);
    }
}
