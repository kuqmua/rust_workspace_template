pub fn build_date_sql_filter(
    optional_table_alias: Option<&crate::sql_identifier::SqlIdentifier>,
    bounds: crate::date_filter_bounds::DateFilterBounds<'_>,
    bind_start: crate::date_sql_bind_start_non_zero_u32::DateSqlBindStartNonZeroU32,
) -> Result<crate::date_sql_filter::DateSqlFilter, crate::date_sql_filter_error::DateSqlFilterError>
{
    let mut bind_index = bind_start.get_inner().get();
    let candidates = [
        (
            constants_str::CREATED_AT,
            constants_str::GREATER_OR_EQUAL,
            bounds.get_created_at_from().copied(),
        ),
        (
            constants_str::CREATED_AT,
            constants_str::LESS_OR_EQUAL,
            bounds.get_created_at_to().copied(),
        ),
        (
            constants_str::UPDATED_AT,
            constants_str::GREATER_OR_EQUAL,
            bounds.get_updated_at_from().copied(),
        ),
        (
            constants_str::UPDATED_AT,
            constants_str::LESS_OR_EQUAL,
            bounds.get_updated_at_to().copied(),
        ),
    ];
    let active_count = candidates
        .iter()
        .filter(|(_, _, value)| value.is_some())
        .count();
    let mut values = Vec::with_capacity(active_count);
    let alias_bytes = optional_table_alias.map_or(constants_usize::ZERO, |alias| {
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
            if let Some(table_alias) = optional_table_alias {
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
            bind_index = bind_index
                .checked_add(1u32)
                .ok_or(crate::date_sql_filter_error::DateSqlFilterError::BindIndexOverflow)?;
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
    fn date_bounds_have_ordered_bind_indices_and_values() {
        let from = chrono::DateTime::parse_from_rfc3339(constants_str::TEST_DATE_SQL_FROM)
            .expect("69ee8323 date_bounds_have_ordered_bind_indices_and_values invariant must hold")
            .to_utc();
        let to = chrono::DateTime::parse_from_rfc3339(constants_str::TEST_DATE_SQL_TO)
            .expect("91eae791 date_bounds_have_ordered_bind_indices_and_values invariant must hold")
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
        .expect("512fa2fb date_bounds_have_ordered_bind_indices_and_values invariant must hold");
        let (fragment, values) = filter.into_parts();
        assert_eq!(fragment.into_inner(), constants_str::TEST_DATE_SQL_FILTER);
        assert_eq!(values.as_ref(), &[from, to]);
    }
}
