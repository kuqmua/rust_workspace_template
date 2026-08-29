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
#[serde(try_from = "crate::pagination_starts_with_zero_raw::PaginationStartsWithZeroRaw")]
#[derive(newtype::FromInner)]
pub struct PaginationStartsWithZero(crate::pagination_base::PaginationBase);

impl PaginationStartsWithZero {
    #[must_use]
    pub fn end(&self) -> crate::pagination_end::PaginationEnd {
        self.0.end()
    }

    #[must_use]
    pub fn start(&self) -> crate::pagination_start::PaginationStart {
        self.0.start()
    }

    pub fn try_new<LimitTy, OffsetTy>(
        limit: LimitTy,
        offset: OffsetTy,
    ) -> Result<
        Self,
        crate::pagination_starts_with_zero_try_new_error::PaginationStartsWithZeroTryNewError,
    >
    where
        LimitTy: Into<crate::pagination_limit::PaginationLimit>,
        OffsetTy: Into<crate::pagination_offset::PaginationOffset>,
    {
        let limit_value = limit.into();
        let offset_value = offset.into();
        if limit_value.get() <= 0 || offset_value.get() < 0 {
            if limit_value.get() <= 0 {
                Err(crate::pagination_starts_with_zero_try_new_error::PaginationStartsWithZeroTryNewError::LimitIsLessThanOrEqToZero {
                    limit: limit_value,
                    location: location_macros::location!(),
                })
            } else {
                Err(crate::pagination_starts_with_zero_try_new_error::PaginationStartsWithZeroTryNewError::OffsetIsLessThanZero {
                    offset: offset_value,
                    location: location_macros::location!(),
                })
            }
        } else if offset_value.get().checked_add(limit_value.get()).is_some() {
            Ok(Self::from(
                crate::pagination_base::PaginationBase::new_unchecked(limit_value, offset_value),
            ))
        } else {
            Err(crate::pagination_starts_with_zero_try_new_error::PaginationStartsWithZeroTryNewError::OffsetPlusLimitIsIntOverflow {
                limit: limit_value,
                offset: offset_value,
                location: location_macros::location!(),
            })
        }
    }
}

impl TryFrom<crate::pagination_starts_with_zero_raw::PaginationStartsWithZeroRaw>
    for PaginationStartsWithZero
{
    type Error =
        crate::pagination_starts_with_zero_try_new_error::PaginationStartsWithZeroTryNewError;

    fn try_from(
        value: crate::pagination_starts_with_zero_raw::PaginationStartsWithZeroRaw,
    ) -> Result<Self, Self::Error> {
        Self::try_new(*value.get_limit(), *value.get_offset())
    }
}

impl<'query_lt> crate::pg_type_where_filter::PgTypeWhereFilter<'query_lt>
    for PaginationStartsWithZero
{
    fn query_bind(
        self,
        query: crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
    ) -> Result<
        crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
        crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    > {
        self.0.query_bind(query)
    }

    fn query_part(
        &self,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
        column: crate::sql_column_ref::SqlColumnRef<'_>,
        add_operator: crate::add_operator::AddOperator,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    > {
        self.0.query_part(increment, column, add_operator)
    }
}

impl crate::default_some_one_element::DefaultSomeOneElement for PaginationStartsWithZero {
    #[inline]
    fn default_some_one_element() -> Self {
        Self::from(crate::pagination_base::PaginationBase::new_unchecked(
            crate::pagination_policy::PaginationPolicy::standard()
                .default_limit()
                .get(),
            0,
        ))
    }
}

impl crate::default_some_one_element_max_page_size::DefaultSomeOneElementMaxPageSize
    for PaginationStartsWithZero
{
    #[inline]
    fn default_some_one_element_max_page_size() -> Self {
        Self::from(crate::pagination_base::PaginationBase::new_unchecked(
            i32::MAX,
            0,
        ))
    }
}
