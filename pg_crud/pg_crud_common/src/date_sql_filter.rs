#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct ChronoUtcDateTimeRef<'value_lt>(&'value_lt chrono::DateTime<chrono::Utc>);

#[derive(Clone, Copy, Debug, Default)]
pub struct DateFilterBounds<'value_lt> {
    created_at_from: Option<ChronoUtcDateTimeRef<'value_lt>>,
    created_at_to: Option<ChronoUtcDateTimeRef<'value_lt>>,
    updated_at_from: Option<ChronoUtcDateTimeRef<'value_lt>>,
    updated_at_to: Option<ChronoUtcDateTimeRef<'value_lt>>,
}
impl<'value_lt> DateFilterBounds<'value_lt> {
    #[must_use]
    pub const fn new(
        created_at_from: Option<ChronoUtcDateTimeRef<'value_lt>>,
        created_at_to: Option<ChronoUtcDateTimeRef<'value_lt>>,
        updated_at_from: Option<ChronoUtcDateTimeRef<'value_lt>>,
        updated_at_to: Option<ChronoUtcDateTimeRef<'value_lt>>,
    ) -> Self {
        Self {
            created_at_from,
            created_at_to,
            updated_at_from,
            updated_at_to,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct StdDateSqlBindStart(std::num::NonZeroU32);

#[derive(Clone, Debug, Eq, PartialEq, newtype::AsRefTarget)]
#[derive(newtype::FromInner)]
pub struct ChronoUtcDateTimes(Vec<chrono::DateTime<chrono::Utc>>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateSqlFilter {
    fragment: crate::QueryPartFragment,
    values: ChronoUtcDateTimes,
}
impl DateSqlFilter {
    #[must_use]
    pub fn into_parts(self) -> (crate::QueryPartFragment, ChronoUtcDateTimes) {
        (self.fragment, self.values)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum DateSqlFilterError {
    #[error("date SQL filter bind index overflowed")]
    BindIndexOverflow,
    #[error("date SQL filter exceeds the query fragment limit")]
    FragmentTooLong,
}

pub fn build_date_sql_filter(
    optional_table_alias: Option<&crate::SqlIdentifier>,
    bounds: DateFilterBounds<'_>,
    bind_start: StdDateSqlBindStart,
) -> Result<DateSqlFilter, DateSqlFilterError> {
    let mut fragment = String::new();
    let mut values = Vec::with_capacity(4usize);
    let mut bind_index = bind_start.0.get();
    let candidates = [
        (
            str_constants::CREATED_AT,
            str_constants::GREATER_OR_EQUAL,
            bounds.created_at_from,
        ),
        (
            str_constants::CREATED_AT,
            str_constants::LESS_OR_EQUAL,
            bounds.created_at_to,
        ),
        (
            str_constants::UPDATED_AT,
            str_constants::GREATER_OR_EQUAL,
            bounds.updated_at_from,
        ),
        (
            str_constants::UPDATED_AT,
            str_constants::LESS_OR_EQUAL,
            bounds.updated_at_to,
        ),
    ];
    candidates
        .into_iter()
        .try_for_each(|(column, comparator, optional_value)| {
            let Some(value) = optional_value else {
                return Ok(());
            };
            if !fragment.is_empty() {
                fragment.push_str(str_constants::AND);
            }
            if let Some(table_alias) = optional_table_alias {
                fragment.push_str(table_alias.as_ref());
                fragment.push('.');
            }
            fragment.push_str(column);
            fragment.push(' ');
            fragment.push_str(comparator);
            fragment.push_str(str_constants::DOLLAR_SIGN);
            std::fmt::Write::write_fmt(&mut fragment, format_args!("{bind_index}"))
                .map_err(|_error| DateSqlFilterError::FragmentTooLong)?;
            values.push(*value.0);
            bind_index = bind_index
                .checked_add(1u32)
                .ok_or(DateSqlFilterError::BindIndexOverflow)?;
            Ok(())
        })?;
    let query_fragment = crate::QueryPartFragment::try_from(fragment)
        .map_err(|_error| DateSqlFilterError::FragmentTooLong)?;
    Ok(DateSqlFilter {
        fragment: query_fragment,
        values: ChronoUtcDateTimes::from(values),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn date_bounds_have_ordered_bind_indices_and_values() {
        let from = chrono::DateTime::parse_from_rfc3339(str_constants::TEST_DATE_SQL_FROM)
            .expect("69ee8323")
            .to_utc();
        let to = chrono::DateTime::parse_from_rfc3339(str_constants::TEST_DATE_SQL_TO)
            .expect("91eae791")
            .to_utc();
        let filter = super::build_date_sql_filter(
            None,
            super::DateFilterBounds::new(Some((&from).into()), Some((&to).into()), None, None),
            std::num::NonZeroU32::MIN.into(),
        )
        .expect("512fa2fb");
        let (fragment, values) = filter.into_parts();
        assert_eq!(fragment.into_inner(), str_constants::TEST_DATE_SQL_FILTER);
        assert_eq!(values.as_ref(), &[from, to]);
    }
}
