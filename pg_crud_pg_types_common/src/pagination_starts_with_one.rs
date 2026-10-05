#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[serde(try_from = "crate::pagination_starts_with_one_raw::PaginationStartsWithOneRaw")]
#[derive(proc_macro_newtype_from_inner::FromInner)]
pub struct PaginationStartsWithOne(pg_crud_common::pagination_base::PaginationBase);

impl Default for PaginationStartsWithOne {
    fn default() -> Self {
        <Self as pg_crud_common::default_some_one_element::DefaultSomeOneElement>::default_some_one_element()
    }
}

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
        l: L,
        o: O,
    ) -> Result<
        Self,
        crate::pagination_starts_with_one_try_new_error::PaginationStartsWithOneTryNewError,
    >
    where
        L: Into<crate::pagination_starts_with_one_value::PaginationStartsWithOneValue>,
        O: Into<crate::pagination_starts_with_one_value::PaginationStartsWithOneValue>,
    {
        let limit_value = l.into();
        let offset_value = o.into();
        if limit_value.get() <= 0 || offset_value.get() < 1 {
            if limit_value.get() <= 0 {
                Err(
                    crate::pagination_starts_with_one_try_new_error::PaginationStartsWithOneTryNewError::LimitIsLessThanOrEqToZero {
                        limit: limit_value,
                        location: proc_macro_location_bang::location!(),
                    },
                )
            } else {
                Err(
                    crate::pagination_starts_with_one_try_new_error::PaginationStartsWithOneTryNewError::OffsetIsLessThanOne {
                        offset: offset_value,
                        location: proc_macro_location_bang::location!(),
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
                    location: proc_macro_location_bang::location!(),
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
        value: crate::pagination_starts_with_one_raw::PaginationStartsWithOneRaw,
    ) -> Result<Self, Self::Error> {
        let (limit, offset) = value.into_parts();
        Self::try_new(limit, offset)
    }
}

impl<'lt> pg_crud_common::pg_type_where_filter::PgTypeWhereFilter<'lt> for PaginationStartsWithOne {
    fn query_bind(
        self,
        sqlx_postgres_query: pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery<'lt>,
    ) -> Result<
        pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery<'lt>,
        pg_crud_common::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    > {
        self.0.query_bind(sqlx_postgres_query)
    }
    fn query_part(
        &self,
        increment: &mut dyn pg_crud_common::query_part_increment_mut::QueryPartIncrementMut,
        sql_column_ref: pg_crud_common::sql_column_ref::SqlColumnRef<'_>,
        add_operator: pg_crud_common::add_operator::AddOperator,
    ) -> Result<
        pg_crud_common::query_part_fragment::QueryPartFragment,
        pg_crud_common::query_part_error::QueryPartError,
    > {
        self.0.query_part(increment, sql_column_ref, add_operator)
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
    fn test_one_based_pagination_deserialization_preserves_valid_inclusive_boundaries() {
        assert!(
            [
                (1i64, 1i64),
                (i64::MAX - 1i64, 1i64),
                (1i64, i64::MAX - 1i64),
                (3i64, 2i64)
            ]
            .into_iter()
            .all(|(limit, offset)| {
                <super::PaginationStartsWithOne as serde::Deserialize>::deserialize(
                    serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                        [
                            (constants_str::OFFSET_ALT, offset),
                            (constants_str::LIMIT, limit),
                        ]
                        .into_iter(),
                    ),
                )
                .is_ok_and(|pagination| {
                    pagination.start().get() == offset && pagination.end().get() == offset + limit
                })
            })
        );
    }

    #[test]
    fn test_one_based_pagination_deserialization_rejects_invalid_values_and_overflow() {
        assert!(
            [
                (0i64, 1i64),
                (-1i64, 1i64),
                (1i64, 0i64),
                (1i64, -1i64),
                (0i64, 0i64),
                (1i64, i64::MAX),
                (i64::MAX, 1i64)
            ]
            .into_iter()
            .all(|(limit, offset)| {
                <super::PaginationStartsWithOne as serde::Deserialize>::deserialize(
                    serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                        [
                            (constants_str::LIMIT, limit),
                            (constants_str::OFFSET_ALT, offset),
                        ]
                        .into_iter(),
                    ),
                )
                .is_err()
            })
        );
    }

    #[test]
    fn test_one_based_pagination_deserialization_rejects_missing_and_duplicate_fields() {
        assert!(
            [
                vec![],
                vec![(constants_str::LIMIT, 1i64)],
                vec![(constants_str::OFFSET_ALT, 1i64)],
                vec![
                    (constants_str::LIMIT, 1i64),
                    (constants_str::LIMIT, 2i64),
                    (constants_str::OFFSET_ALT, 1i64)
                ],
                vec![
                    (constants_str::LIMIT, 1i64),
                    (constants_str::OFFSET_ALT, 1i64),
                    (constants_str::OFFSET_ALT, 2i64)
                ],
            ]
            .into_iter()
            .all(|fields| {
                <super::PaginationStartsWithOne as serde::Deserialize>::deserialize(
                    serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                        fields.into_iter(),
                    ),
                )
                .is_err()
            })
        );
    }

    #[test]
    fn test_pagination_starts_with_one_accepts_inclusive_boundaries() {
        let pagination = super::PaginationStartsWithOne::try_new(2i64, constants_i64::ONE)
            .expect(constants_str::DIAGNOSTIC_007C805E);
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
        let default = super::PaginationStartsWithOne::default();
        assert_eq!(default.start().get(), constants_i64::ONE);
        assert_eq!(
            default.end().get(),
            pg_crud_common::pagination_policy::PaginationPolicy::standard()
                .default_limit()
                .get()
                + constants_i64::ONE
        );
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
