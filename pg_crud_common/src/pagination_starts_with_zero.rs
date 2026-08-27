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
#[serde(
    try_from = "crate::domain_types::pagination_starts_with_zero_raw::PaginationStartsWithZeroRaw"
)]
#[derive(newtype::FromInner)]
pub struct PaginationStartsWithZero(crate::domain_types::PaginationBase);

impl PaginationStartsWithZero {
    #[must_use]
    pub fn end(&self) -> crate::domain_types::PaginationEnd {
        self.0.end()
    }

    #[must_use]
    pub fn start(&self) -> crate::domain_types::PaginationStart {
        self.0.start()
    }

    pub fn try_new<LimitTy, OffsetTy>(
        limit: LimitTy,
        offset: OffsetTy,
    ) -> Result<Self, crate::domain_types::PaginationStartsWithZeroTryNewError>
    where
        LimitTy: Into<crate::domain_types::PaginationLimit>,
        OffsetTy: Into<crate::domain_types::PaginationOffset>,
    {
        let limit_value = limit.into();
        let offset_value = offset.into();
        if limit_value.get() <= 0 || offset_value.get() < 0 {
            if limit_value.get() <= 0 {
                Err(crate::domain_types::PaginationStartsWithZeroTryNewError::LimitIsLessThanOrEqToZero {
                    limit: limit_value,
                    location: location_macros::location!(),
                })
            } else {
                Err(crate::domain_types::PaginationStartsWithZeroTryNewError::OffsetIsLessThanZero {
                    offset: offset_value,
                    location: location_macros::location!(),
                })
            }
        } else if offset_value.get().checked_add(limit_value.get()).is_some() {
            Ok(Self::from(
                crate::domain_types::PaginationBase::new_unchecked(limit_value, offset_value),
            ))
        } else {
            Err(crate::domain_types::PaginationStartsWithZeroTryNewError::OffsetPlusLimitIsIntOverflow {
                limit: limit_value,
                offset: offset_value,
                location: location_macros::location!(),
            })
        }
    }
}

impl TryFrom<crate::domain_types::pagination_starts_with_zero_raw::PaginationStartsWithZeroRaw>
    for PaginationStartsWithZero
{
    type Error = crate::domain_types::PaginationStartsWithZeroTryNewError;

    fn try_from(
        value: crate::domain_types::pagination_starts_with_zero_raw::PaginationStartsWithZeroRaw,
    ) -> Result<Self, Self::Error> {
        Self::try_new(value.limit, value.offset)
    }
}

impl<'query_lt> crate::domain_types::PgTypeWhereFilter<'query_lt> for PaginationStartsWithZero {
    fn query_bind(
        self,
        query: crate::domain_types::SqlxPostgresQuery<'query_lt>,
    ) -> Result<
        crate::domain_types::SqlxPostgresQuery<'query_lt>,
        crate::domain_types::SqlxPostgresQueryBindError,
    > {
        self.0.query_bind(query)
    }

    fn query_part(
        &self,
        increment: &mut dyn crate::domain_types::QueryPartIncrementMut,
        column: crate::domain_types::SqlColumnRef<'_>,
        add_operator: crate::domain_types::AddOperator,
    ) -> Result<crate::domain_types::QueryPartFragment, crate::domain_types::QueryPartError> {
        self.0.query_part(increment, column, add_operator)
    }
}

impl crate::domain_types::DefaultSomeOneElement for PaginationStartsWithZero {
    #[inline]
    fn default_some_one_element() -> Self {
        Self::from(crate::domain_types::PaginationBase::new_unchecked(
            crate::domain_types::PaginationPolicy::standard()
                .default_limit()
                .get(),
            0,
        ))
    }
}

impl crate::domain_types::DefaultSomeOneElementMaxPageSize for PaginationStartsWithZero {
    #[inline]
    fn default_some_one_element_max_page_size() -> Self {
        Self::from(crate::domain_types::PaginationBase::new_unchecked(
            i32::MAX,
            0,
        ))
    }
}
