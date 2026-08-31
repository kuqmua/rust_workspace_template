#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
)]
#[serde(try_from = "crate::pagination_starts_with_one_raw::PaginationStartsWithOneRaw")]
#[derive(newtype::FromInner)]
pub struct PaginationStartsWithOne(pg_crud_common::pagination_base::PaginationBase);

impl PaginationStartsWithOne {
    #[must_use]
    pub fn end(&self) -> crate::pagination_starts_with_one_value::PaginationStartsWithOneValue {
        crate::pagination_starts_with_one_value::PaginationStartsWithOneValue::from(
            self.0.end().get(),
        )
    }
    #[must_use]
    pub fn start(&self) -> crate::pagination_starts_with_one_value::PaginationStartsWithOneValue {
        crate::pagination_starts_with_one_value::PaginationStartsWithOneValue::from(
            self.0.start().get(),
        )
    }
    pub fn try_new<L, O>(
        limit: L,
        offset: O,
    ) -> Result<
        Self,
        crate::pagination_starts_with_one_try_new_error::PaginationStartsWithOneTryNewError,
    >
    where
        L: Into<crate::pagination_starts_with_one_value::PaginationStartsWithOneValue>,
        O: Into<crate::pagination_starts_with_one_value::PaginationStartsWithOneValue>,
    {
        let limit_value = limit.into();
        let offset_value = offset.into();
        if limit_value.get() <= 0 || offset_value.get() < 1 {
            if limit_value.get() <= 0 {
                Err(
                    crate::pagination_starts_with_one_try_new_error::PaginationStartsWithOneTryNewError::LimitIsLessThanOrEqToZero {
                        limit: limit_value,
                        location: location_macros::location!(),
                    },
                )
            } else {
                Err(
                    crate::pagination_starts_with_one_try_new_error::PaginationStartsWithOneTryNewError::OffsetIsLessThanOne {
                        offset: offset_value,
                        location: location_macros::location!(),
                    },
                )
            }
        } else if offset_value.get().checked_add(limit_value.get()).is_some() {
            Ok(Self::from(
                pg_crud_common::pagination_base::PaginationBase::new_unchecked(
                    limit_value.get(),
                    offset_value.get(),
                ),
            ))
        } else {
            Err(
                crate::pagination_starts_with_one_try_new_error::PaginationStartsWithOneTryNewError::OffsetPlusLimitIsIntOverflow {
                    limit: limit_value,
                    offset: offset_value,
                    location: location_macros::location!(),
                },
            )
        }
    }
}

impl TryFrom<crate::pagination_starts_with_one_raw::PaginationStartsWithOneRaw>
    for PaginationStartsWithOne
{
    type Error =
        crate::pagination_starts_with_one_try_new_error::PaginationStartsWithOneTryNewError;
    fn try_from(
        v: crate::pagination_starts_with_one_raw::PaginationStartsWithOneRaw,
    ) -> Result<Self, Self::Error> {
        let (limit, offset) = v.into_parts();
        Self::try_new(limit, offset)
    }
}

impl<'lt> pg_crud_common::pg_type_where_filter::PgTypeWhereFilter<'lt> for PaginationStartsWithOne {
    fn query_bind(
        self,
        query: pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery<'lt>,
    ) -> Result<
        pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery<'lt>,
        pg_crud_common::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    > {
        self.0.query_bind(query)
    }
    fn query_part(
        &self,
        increment: &mut dyn pg_crud_common::query_part_increment_mut::QueryPartIncrementMut,
        column: pg_crud_common::sql_column_ref::SqlColumnRef<'_>,
        add_operator: pg_crud_common::add_operator::AddOperator,
    ) -> Result<
        pg_crud_common::query_part_fragment::QueryPartFragment,
        pg_crud_common::query_part_error::QueryPartError,
    > {
        self.0.query_part(increment, column, add_operator)
    }
}

impl pg_crud_common::default_some_one_element::DefaultSomeOneElement for PaginationStartsWithOne {
    #[inline]
    fn default_some_one_element() -> Self {
        Self::from(
            pg_crud_common::pagination_base::PaginationBase::new_unchecked(
                pg_crud_common::pagination_policy::PaginationPolicy::standard()
                    .default_limit()
                    .get(),
                1,
            ),
        )
    }
}

impl pg_crud_common::default_some_one_element_max_page_size::DefaultSomeOneElementMaxPageSize
    for PaginationStartsWithOne
{
    #[inline]
    fn default_some_one_element_max_page_size() -> Self {
        let one: i32 = 1;
        Self::from(
            pg_crud_common::pagination_base::PaginationBase::new_unchecked(i32::MAX - one, one),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pagination_starts_with_one_accepts_inclusive_boundaries() {
        let pagination = super::PaginationStartsWithOne::try_new(2i64, constants_i64::ONE).expect(
            "007c805e pagination_starts_with_one_accepts_inclusive_boundaries invariant must hold",
        );
        assert_eq!(pagination.start().get(), constants_i64::ONE);
        assert_eq!(pagination.end().get(), 3i64);
    }

    #[test]
    fn test_pagination_starts_with_one_distinguishes_validation_errors() {
        assert!(matches!(
            super::PaginationStartsWithOne::try_new(constants_i64::ZERO, constants_i64::ONE),
            Err(crate::pagination_starts_with_one_try_new_error::PaginationStartsWithOneTryNewError::LimitIsLessThanOrEqToZero { .. })
        ));
        assert!(matches!(
            super::PaginationStartsWithOne::try_new(constants_i64::ONE, constants_i64::ZERO),
            Err(crate::pagination_starts_with_one_try_new_error::PaginationStartsWithOneTryNewError::OffsetIsLessThanOne { .. })
        ));
        assert!(matches!(
            super::PaginationStartsWithOne::try_new(constants_i64::ONE, i64::MAX),
            Err(crate::pagination_starts_with_one_try_new_error::PaginationStartsWithOneTryNewError::OffsetPlusLimitIsIntOverflow { .. })
        ));
    }

    #[test]
    fn test_pagination_defaults_start_at_one_and_use_the_expected_limits() {
        let standard =
            <super::PaginationStartsWithOne as pg_crud_common::default_some_one_element::DefaultSomeOneElement>::default_some_one_element();
        assert_eq!(standard.start().get(), constants_i64::ONE);
        assert_eq!(
            standard.end().get(),
            pg_crud_common::pagination_policy::PaginationPolicy::standard()
                .default_limit()
                .get()
                + constants_i64::ONE
        );
        let maximum =
            <super::PaginationStartsWithOne as pg_crud_common::default_some_one_element_max_page_size::DefaultSomeOneElementMaxPageSize>::default_some_one_element_max_page_size();
        assert_eq!(maximum.start().get(), constants_i64::ONE);
        assert_eq!(maximum.end().get(), i64::from(i32::MAX));
    }
}
