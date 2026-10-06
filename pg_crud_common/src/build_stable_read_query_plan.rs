pub fn build_stable_read_query_plan(
    query_part_fragment: crate::query_part_fragment::QueryPartFragment,
    sort_column: &crate::sql_identifier::SqlIdentifier,
    tie_break_column: &crate::sql_identifier::SqlIdentifier,
    query_sort_order: crate::query_sort_order::QuerySortOrder,
    limit_bind: crate::read_query_bind_index_non_zero_u32::ReadQueryBindIndexNonZeroU32,
    offset_bind: crate::read_query_bind_index_non_zero_u32::ReadQueryBindIndexNonZeroU32,
) -> Result<crate::read_query_plan::ReadQueryPlan, crate::read_query_plan_error::ReadQueryPlanError>
{
    let mut query = query_part_fragment.into_inner();
    let order_sql = query_sort_order.sql();
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
    #[test]
    fn test_stable_query_modes_columns_and_bind_boundaries_preserve_sql() {
        let sort_column = stable_read_identifier(constants_str::CREATED_AT);
        let different_column = stable_read_identifier(constants_str::SQL_NAMES_ID);
        assert!([
            (crate::query_sort_order::QuerySortOrder::Ascending, constants_str::SORT_ASC),
            (crate::query_sort_order::QuerySortOrder::Descending, constants_str::SORT_DESC),
        ].into_iter().all(|(query_sort_order, order_sql)| {
            [false, true].into_iter().all(|same_column| {
                let tie_break_column = if same_column { &sort_column } else { &different_column };
                [
                    (std::num::NonZeroU32::MIN, std::num::NonZeroU32::MAX),
                    (std::num::NonZeroU32::MAX, std::num::NonZeroU32::MIN),
                    (std::num::NonZeroU32::MAX, std::num::NonZeroU32::MAX),
                ].into_iter().all(|(limit_bind, offset_bind)| {
                    let tie_break = if same_column {
                        String::new()
                    } else {
                        format!("{}{} {order_sql}", constants_str::TEXT_ALT_6, constants_str::SQL_NAMES_ID)
                    };
                    let expected = format!("{}{}{} {order_sql}{tie_break}{}{limit_bind}{}{offset_bind}",
                        constants_str::TEST_READ_QUERY_BASE, constants_str::READ_ORDER_BY,
                        constants_str::CREATED_AT, constants_str::LIMIT_DOLLAR, constants_str::OFFSET_DOLLAR);
                    crate::query_part_fragment::QueryPartFragment::try_from(constants_str::TEST_READ_QUERY_BASE.to_owned())
                        .is_ok_and(|query_part_fragment| {
                            crate::build_stable_read_query_plan::build_stable_read_query_plan(
                                query_part_fragment, &sort_column, tie_break_column, query_sort_order,
                                crate::read_query_bind_index_non_zero_u32::ReadQueryBindIndexNonZeroU32::from(limit_bind),
                                crate::read_query_bind_index_non_zero_u32::ReadQueryBindIndexNonZeroU32::from(offset_bind),
                            ).is_ok_and(|read_query_plan| crate::query_part_fragment::QueryPartFragment::from(read_query_plan).as_ref() == expected)
                        })
                })
            })
        }));
    }

    #[test]
    fn test_stable_query_output_limit_covers_every_suffix_append_stage() {
        let column = stable_read_identifier(constants_str::SQL_NAMES_ID);
        let bind = crate::read_query_bind_index_non_zero_u32::ReadQueryBindIndexNonZeroU32::from(
            std::num::NonZeroU32::MAX,
        );
        let initial_suffix = format!(
            "{}{} {}{}",
            constants_str::READ_ORDER_BY,
            constants_str::SQL_NAMES_ID,
            constants_str::SORT_ASC,
            constants_str::LIMIT_DOLLAR
        );
        let digits = std::num::NonZeroU32::MAX.to_string();
        let complete_suffix = format!(
            "{initial_suffix}{digits}{}{digits}",
            constants_str::OFFSET_DOLLAR
        );
        let padded_fragment = |length| {
            let mut base = constants_str::TEST_READ_QUERY_BASE.to_owned();
            base.extend(std::iter::repeat_n(' ', length - base.len()));
            crate::query_part_fragment::QueryPartFragment::try_from(base)
        };
        let build = |query_part_fragment: crate::query_part_fragment::QueryPartFragment| {
            crate::build_stable_read_query_plan::build_stable_read_query_plan(
                query_part_fragment,
                &column,
                &column,
                crate::query_sort_order::QuerySortOrder::Ascending,
                bind,
                bind,
            )
        };
        let maximum = crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN;
        assert!(padded_fragment(maximum - complete_suffix.len()).is_ok_and(
            |query_part_fragment| {
                build(query_part_fragment).is_ok_and(|read_query_plan| {
                    let fragment =
                        crate::query_part_fragment::QueryPartFragment::from(read_query_plan);
                    fragment.as_ref().len() == maximum
                        && fragment
                            .as_ref()
                            .starts_with(constants_str::TEST_READ_QUERY_BASE)
                        && fragment.as_ref().ends_with(&complete_suffix)
                })
            }
        ));
        assert!(
            [
                initial_suffix.len(),
                initial_suffix.len() + digits.len(),
                initial_suffix.len() + digits.len() + constants_str::OFFSET_DOLLAR.len(),
                complete_suffix.len(),
            ]
            .into_iter()
            .all(|stage_length| {
                padded_fragment(maximum - stage_length + 1usize).is_ok_and(|query_part_fragment| {
                    build(query_part_fragment)
                        == Err(crate::read_query_plan_error::ReadQueryPlanError::TooManyFragments)
                })
            })
        );
    }

    fn stable_read_identifier(str: &str) -> crate::sql_identifier::SqlIdentifier {
        crate::sql_identifier::SqlIdentifier::try_from(str.to_owned())
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
