#[derive(
    Debug, serde::Deserialize, schemars::JsonSchema, optimal_memory_layout::OptimalMemoryLayout,
)]
struct PaginationStartsWithOneRaw {
    limit: PaginationStartsWithOneValue,
    offset: PaginationStartsWithOneValue,
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::Display,
    newtype::FromInner,
    newtype::GetInner,
    newtype::ToErrString,
)]
#[serde(from = "i64")]
pub struct PaginationStartsWithOneValue(i64);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct IsPrimaryKey(bool);
impl From<pg_crud_common::IsPrimaryKey> for IsPrimaryKey {
    fn from(value: pg_crud_common::IsPrimaryKey) -> Self {
        Self::from(bool::from(value))
    }
}
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
#[serde(try_from = "PaginationStartsWithOneRaw")]
#[derive(newtype::FromInner)]
pub struct PaginationStartsWithOne(pg_crud_common::PaginationBase);

#[location::errors_with_location]
#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum PaginationStartsWithOneTryNewError {
    LimitIsLessThanOrEqToZero {
        #[eo_to_err_string_serde]
        limit: PaginationStartsWithOneValue,
    },
    OffsetIsLessThanOne {
        #[eo_to_err_string_serde]
        offset: PaginationStartsWithOneValue,
    },
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_err_string_serde]
        limit: PaginationStartsWithOneValue,
        #[eo_to_err_string_serde]
        offset: PaginationStartsWithOneValue,
    },
}
impl PaginationStartsWithOne {
    #[must_use]
    pub fn end(&self) -> PaginationStartsWithOneValue {
        PaginationStartsWithOneValue::from(self.0.end().get())
    }
    #[must_use]
    pub fn start(&self) -> PaginationStartsWithOneValue {
        PaginationStartsWithOneValue::from(self.0.start().get())
    }
    pub fn try_new<L, O>(limit: L, offset: O) -> Result<Self, PaginationStartsWithOneTryNewError>
    where
        L: Into<PaginationStartsWithOneValue>,
        O: Into<PaginationStartsWithOneValue>,
    {
        let limit_value = limit.into();
        let offset_value = offset.into();
        if limit_value.get() <= 0 || offset_value.get() < 1 {
            if limit_value.get() <= 0 {
                Err(
                    PaginationStartsWithOneTryNewError::LimitIsLessThanOrEqToZero {
                        limit: limit_value,
                        location: location_macros::location!(),
                    },
                )
            } else {
                Err(PaginationStartsWithOneTryNewError::OffsetIsLessThanOne {
                    offset: offset_value,
                    location: location_macros::location!(),
                })
            }
        } else if offset_value.get().checked_add(limit_value.get()).is_some() {
            Ok(Self::from(pg_crud_common::PaginationBase::new_unchecked(
                limit_value.get(),
                offset_value.get(),
            )))
        } else {
            Err(
                PaginationStartsWithOneTryNewError::OffsetPlusLimitIsIntOverflow {
                    limit: limit_value,
                    offset: offset_value,
                    location: location_macros::location!(),
                },
            )
        }
    }
}
impl TryFrom<PaginationStartsWithOneRaw> for PaginationStartsWithOne {
    type Error = PaginationStartsWithOneTryNewError;
    fn try_from(v: PaginationStartsWithOneRaw) -> Result<Self, Self::Error> {
        Self::try_new(v.limit, v.offset)
    }
}
impl<'lt> pg_crud_common::PgTypeWhereFilter<'lt> for PaginationStartsWithOne {
    fn query_bind(
        self,
        query: pg_crud_common::SqlxPostgresQuery<'lt>,
    ) -> Result<pg_crud_common::SqlxPostgresQuery<'lt>, pg_crud_common::SqlxPostgresQueryBindError>
    {
        self.0.query_bind(query)
    }
    fn query_part(
        &self,
        increment: &mut dyn pg_crud_common::QueryPartIncrementMut,
        column: pg_crud_common::SqlColumnRef<'_>,
        add_operator: pg_crud_common::AddOperator,
    ) -> Result<pg_crud_common::QueryPartFragment, pg_crud_common::QueryPartError> {
        self.0.query_part(increment, column, add_operator)
    }
}
impl pg_crud_common::DefaultSomeOneElement for PaginationStartsWithOne {
    #[inline]
    fn default_some_one_element() -> Self {
        Self::from(pg_crud_common::PaginationBase::new_unchecked(
            pg_crud_common::PaginationPolicy::standard()
                .default_limit()
                .get(),
            1,
        ))
    }
}
impl pg_crud_common::DefaultSomeOneElementMaxPageSize for PaginationStartsWithOne {
    #[inline]
    fn default_some_one_element_max_page_size() -> Self {
        let one: i32 = 1;
        Self::from(pg_crud_common::PaginationBase::new_unchecked(
            i32::MAX - one,
            one,
        ))
    }
}
#[must_use]
pub fn maybe_primary_key<V>(v: V) -> impl std::fmt::Display
where
    V: Into<IsPrimaryKey>,
{
    if bool::from(v.into()) {
        constants_str::PRIMARY_KEY
    } else {
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pagination_starts_with_one_accepts_inclusive_boundaries() {
        let pagination = super::PaginationStartsWithOne::try_new(2i64, constants_i64::ONE).expect(
            "007c805e pagination_starts_with_one_accepts_inclusive_boundaries invariant must hold",
        );
        assert_eq!(pagination.start().get(), constants_i64::ONE);
        assert_eq!(pagination.end().get(), 3i64);
    }

    #[test]
    fn pagination_starts_with_one_distinguishes_validation_errors() {
        assert!(matches!(
            super::PaginationStartsWithOne::try_new(constants_i64::ZERO, constants_i64::ONE),
            Err(super::PaginationStartsWithOneTryNewError::LimitIsLessThanOrEqToZero { .. })
        ));
        assert!(matches!(
            super::PaginationStartsWithOne::try_new(constants_i64::ONE, constants_i64::ZERO),
            Err(super::PaginationStartsWithOneTryNewError::OffsetIsLessThanOne { .. })
        ));
        assert!(matches!(
            super::PaginationStartsWithOne::try_new(constants_i64::ONE, i64::MAX),
            Err(super::PaginationStartsWithOneTryNewError::OffsetPlusLimitIsIntOverflow { .. })
        ));
    }

    #[test]
    fn pagination_defaults_start_at_one_and_use_the_expected_limits() {
        let standard =
            <super::PaginationStartsWithOne as pg_crud_common::DefaultSomeOneElement>::default_some_one_element();
        assert_eq!(standard.start().get(), constants_i64::ONE);
        assert_eq!(
            standard.end().get(),
            pg_crud_common::PaginationPolicy::standard()
                .default_limit()
                .get()
                + constants_i64::ONE
        );
        let maximum =
            <super::PaginationStartsWithOne as pg_crud_common::DefaultSomeOneElementMaxPageSize>::default_some_one_element_max_page_size();
        assert_eq!(maximum.start().get(), constants_i64::ONE);
        assert_eq!(maximum.end().get(), i64::from(i32::MAX));
    }

    #[test]
    fn primary_key_suffix_matches_the_typed_flag() {
        assert_eq!(
            super::maybe_primary_key(super::IsPrimaryKey::from(true)).to_string(),
            constants_str::PRIMARY_KEY
        );
        assert_eq!(
            super::maybe_primary_key(super::IsPrimaryKey::from(false)).to_string(),
            constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
        );
        assert_eq!(
            super::maybe_primary_key(pg_crud_common::IsPrimaryKey::from(true)).to_string(),
            constants_str::PRIMARY_KEY
        );
    }
}
