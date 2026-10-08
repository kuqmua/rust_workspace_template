pub fn build_date_sql_filter(
    option: Option<&crate::sql_identifier::SqlIdentifier>,
    date_filter_bounds: crate::date_filter_bounds::DateFilterBounds<'_>,
    date_sql_bind_start_non_zero_u32: crate::date_sql_bind_start_non_zero_u32::DateSqlBindStartNonZeroU32,
) -> Result<crate::date_sql_filter::DateSqlFilter, crate::date_sql_filter_error::DateSqlFilterError>
{
    let mut bind_index = date_sql_bind_start_non_zero_u32.get_inner().get();
    let candidates = [
        (
            constants_str::CREATED_AT,
            constants_str::GREATER_OR_EQUAL,
            date_filter_bounds.get_created_at_from().copied(),
        ),
        (
            constants_str::CREATED_AT,
            constants_str::LESS_OR_EQUAL,
            date_filter_bounds.get_created_at_to().copied(),
        ),
        (
            constants_str::UPDATED_AT,
            constants_str::GREATER_OR_EQUAL,
            date_filter_bounds.get_updated_at_from().copied(),
        ),
        (
            constants_str::UPDATED_AT,
            constants_str::LESS_OR_EQUAL,
            date_filter_bounds.get_updated_at_to().copied(),
        ),
    ];
    let active_count = candidates
        .iter()
        .filter(|(_, _, value)| value.is_some())
        .count();
    let mut values = Vec::with_capacity(active_count);
    let alias_bytes = option.map_or(constants_usize::ZERO, |alias| {
        alias.as_ref().len().saturating_add(constants_usize::ONE)
    });
    let fragment_capacity = candidates
        .iter()
        .filter(|(_, _, value)| value.is_some())
        .map(|(column, comparator, _)| {
            alias_bytes
                .saturating_add(column.len())
                .saturating_add(constants_usize::ONE)
                .saturating_add(comparator.len())
                .saturating_add(constants_str::DOLLAR_SIGN.len())
                .saturating_add(10usize)
        })
        .sum::<usize>()
        .saturating_add(
            active_count
                .saturating_sub(constants_usize::ONE)
                .saturating_mul(constants_str::AND.len()),
        );
    let mut fragment = String::with_capacity(fragment_capacity);
    candidates
        .into_iter()
        .try_for_each(|(column, comparator, optional_value)| {
            let Some(value) = optional_value else {
                return Ok(());
            };
            if !fragment.is_empty() {
                fragment.push_str(constants_str::AND);
            }
            if let Some(table_alias) = option {
                fragment.push_str(table_alias.as_ref());
                fragment.push('.');
            }
            fragment.push_str(column);
            fragment.push(' ');
            fragment.push_str(comparator);
            fragment.push_str(constants_str::DOLLAR_SIGN);
            std::fmt::Write::write_fmt(&mut fragment, format_args!("{bind_index}")).map_err(
                |_error| crate::date_sql_filter_error::DateSqlFilterError::FragmentTooLong,
            )?;
            values.push(**value.get_inner());
            if values.len() < active_count {
                bind_index = bind_index
                    .checked_add(1u32)
                    .ok_or(crate::date_sql_filter_error::DateSqlFilterError::BindIndexOverflow)?;
            }
            Ok(())
        })?;
    let query_fragment = crate::query_part_fragment::QueryPartFragment::try_from(fragment)
        .map_err(|_error| crate::date_sql_filter_error::DateSqlFilterError::FragmentTooLong)?;
    Ok(crate::date_sql_filter::DateSqlFilter::new(
        query_fragment,
        crate::chrono_utc_date_times::ChronoUtcDateTimes::from(values),
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_empty_date_filter_preserves_empty_sql_with_optional_alias() {
        assert!(
            crate::sql_identifier::SqlIdentifier::try_from(String::from(constants_str::X))
                .is_ok_and(
                    |identifier| [None, Some(&identifier)].into_iter().all(|alias| {
                        crate::build_date_sql_filter::build_date_sql_filter(
                            alias,
                            crate::date_filter_bounds::DateFilterBounds::new(
                                None, None, None, None,
                            ),
                            std::num::NonZeroU32::MAX.into(),
                        )
                        .is_ok_and(|filter| {
                            filter.get_fragment().as_ref() == constants_str::EMPTY
                                && filter.get_values().as_ref().is_empty()
                        })
                    })
                )
        );
    }

    #[test]
    fn test_aliased_date_filter_preserves_all_columns_values_and_final_bind_index() {
        assert!(
            crate::sql_identifier::SqlIdentifier::try_from(String::from(constants_str::X))
                .is_ok_and(|identifier| {
                    let created_from = chrono::DateTime::<chrono::Utc>::MIN_UTC;
                    let created_to = chrono::DateTime::<chrono::Utc>::UNIX_EPOCH;
                    let updated_from = chrono::DateTime::<chrono::Utc>::MAX_UTC;
                    let updated_to = chrono::DateTime::<chrono::Utc>::MIN_UTC;
                    std::num::NonZeroU32::new(u32::MAX - 3).is_some_and(|start| {
                        let render_expected_term = |column, comparator, bind_index| {
                            format!(
                                "{}.{} {}{}{}",
                                identifier.as_ref(),
                                column,
                                comparator,
                                constants_str::DOLLAR_SIGN,
                                bind_index
                            )
                        };
                        let expected = [
                            render_expected_term(
                                constants_str::CREATED_AT,
                                constants_str::GREATER_OR_EQUAL,
                                u32::MAX - 3,
                            ),
                            render_expected_term(
                                constants_str::CREATED_AT,
                                constants_str::LESS_OR_EQUAL,
                                u32::MAX - 2,
                            ),
                            render_expected_term(
                                constants_str::UPDATED_AT,
                                constants_str::GREATER_OR_EQUAL,
                                u32::MAX - 1,
                            ),
                            render_expected_term(
                                constants_str::UPDATED_AT,
                                constants_str::LESS_OR_EQUAL,
                                u32::MAX,
                            ),
                        ]
                        .join(constants_str::AND);
                        crate::build_date_sql_filter::build_date_sql_filter(
                            Some(&identifier),
                            crate::date_filter_bounds::DateFilterBounds::new(
                                Some((&created_from).into()),
                                Some((&created_to).into()),
                                Some((&updated_from).into()),
                                Some((&updated_to).into()),
                            ),
                            start.into(),
                        )
                        .is_ok_and(|filter| {
                            filter.get_fragment().as_ref() == expected
                                && filter.get_values().as_ref()
                                    == [created_from, created_to, updated_from, updated_to]
                        })
                    })
                })
        );
    }

    #[test]
    fn test_date_bounds_have_ordered_bind_indices_and_values() {
        let from = chrono::DateTime::parse_from_rfc3339(constants_str::TEST_DATE_SQL_FROM)
            .expect(constants_str::DIAGNOSTIC_69EE8323)
            .to_utc();
        let to = chrono::DateTime::parse_from_rfc3339(constants_str::TEST_DATE_SQL_TO)
            .expect(constants_str::DIAGNOSTIC_91EAE791)
            .to_utc();
        let filter = crate::build_date_sql_filter::build_date_sql_filter(
            None,
            crate::date_filter_bounds::DateFilterBounds::new(
                Some((&from).into()),
                Some((&to).into()),
                None,
                None,
            ),
            std::num::NonZeroU32::MIN.into(),
        )
        .expect(constants_str::DIAGNOSTIC_512FA2FB);
        let (fragment, values) = filter.into_parts();
        assert_eq!(fragment.into_inner(), constants_str::TEST_DATE_SQL_FILTER);
        assert_eq!(values.as_ref(), &[from, to]);
    }

    #[test]
    fn test_final_date_bound_accepts_last_representable_bind_index() {
        let from = chrono::DateTime::<chrono::Utc>::UNIX_EPOCH;
        assert!(matches!(
            crate::build_date_sql_filter::build_date_sql_filter(
                None,
                crate::date_filter_bounds::DateFilterBounds::new(
                    Some((&from).into()),
                    None,
                    None,
                    None,
                ),
                std::num::NonZeroU32::MAX.into(),
            ),
            Ok(filter) if filter.get_values().as_ref() == [from]
        ));
    }

    #[test]
    fn test_additional_date_bound_rejects_bind_index_overflow() {
        let from = chrono::DateTime::<chrono::Utc>::UNIX_EPOCH;
        assert_eq!(
            crate::build_date_sql_filter::build_date_sql_filter(
                None,
                crate::date_filter_bounds::DateFilterBounds::new(
                    Some((&from).into()),
                    Some((&from).into()),
                    None,
                    None,
                ),
                std::num::NonZeroU32::MAX.into(),
            ),
            Err(crate::date_sql_filter_error::DateSqlFilterError::BindIndexOverflow)
        );
    }

    #[test]
    fn test_sparse_date_bounds_preserve_sql_and_bind_value_order() {
        assert!(
            chrono::DateTime::<chrono::Utc>::from_timestamp(1i64, 0u32).is_some_and(|last| {
                let dates = [
                    chrono::DateTime::<chrono::Utc>::MIN_UTC,
                    chrono::DateTime::<chrono::Utc>::UNIX_EPOCH,
                    chrono::DateTime::<chrono::Utc>::MAX_UTC,
                    last,
                ];
                crate::sql_identifier::SqlIdentifier::try_from(constants_str::X.to_owned())
                    .is_ok_and(|identifier| {
                        (0u8..16u8).all(|mask| {
                            let enabled = std::array::from_fn::<_, 4usize, _>(|index| {
                                mask & (1u8 << index) != 0u8
                            });
                            [None, Some(&identifier)].into_iter().all(|alias| {
                                let prefix = alias.map_or_else(String::new, |table_alias| {
                                    format!("{}.", table_alias.as_ref())
                                });
                                let terms = [
                                    (
                                        constants_str::CREATED_AT,
                                        constants_str::GREATER_OR_EQUAL,
                                        enabled[0],
                                    ),
                                    (
                                        constants_str::CREATED_AT,
                                        constants_str::LESS_OR_EQUAL,
                                        enabled[1],
                                    ),
                                    (
                                        constants_str::UPDATED_AT,
                                        constants_str::GREATER_OR_EQUAL,
                                        enabled[2],
                                    ),
                                    (
                                        constants_str::UPDATED_AT,
                                        constants_str::LESS_OR_EQUAL,
                                        enabled[3],
                                    ),
                                ];
                                let expected_sql = terms
                                    .into_iter()
                                    .filter(|(_, _, active)| *active)
                                    .zip(1usize..)
                                    .map(|((column, comparator, _), bind)| {
                                        format!(
                                            "{prefix}{column} {comparator}{}{bind}",
                                            constants_str::DOLLAR_SIGN
                                        )
                                    })
                                    .collect::<Vec<_>>()
                                    .join(constants_str::AND);
                                let expected_values = dates
                                    .iter()
                                    .zip(enabled)
                                    .filter(|(_, active)| *active)
                                    .map(|(date, _)| *date)
                                    .collect::<Vec<_>>();
                                crate::build_date_sql_filter::build_date_sql_filter(
                                    alias,
                                    crate::date_filter_bounds::DateFilterBounds::new(
                                        enabled[0].then(|| (&dates[0]).into()),
                                        enabled[1].then(|| (&dates[1]).into()),
                                        enabled[2].then(|| (&dates[2]).into()),
                                        enabled[3].then(|| (&dates[3]).into()),
                                    ),
                                    std::num::NonZeroU32::MIN.into(),
                                )
                                .is_ok_and(|filter| {
                                    filter.get_fragment().as_ref() == expected_sql
                                        && filter.get_values().as_ref() == expected_values
                                })
                            })
                        })
                    })
            })
        );
    }
}
