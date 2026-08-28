use super::pagination_starts_with_one_raw::PaginationStartsWithOneRaw;

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
pub struct PaginationStartsWithOne(pg_crud_common::domain_types::PaginationBase);

impl PaginationStartsWithOne {
    #[must_use]
    pub fn end(&self) -> super::PaginationStartsWithOneValue {
        super::PaginationStartsWithOneValue::from(self.0.end().get())
    }
    #[must_use]
    pub fn start(&self) -> super::PaginationStartsWithOneValue {
        super::PaginationStartsWithOneValue::from(self.0.start().get())
    }
    pub fn try_new<L, O>(
        limit: L,
        offset: O,
    ) -> Result<Self, super::PaginationStartsWithOneTryNewError>
    where
        L: Into<super::PaginationStartsWithOneValue>,
        O: Into<super::PaginationStartsWithOneValue>,
    {
        let limit_value = limit.into();
        let offset_value = offset.into();
        if limit_value.get() <= 0 || offset_value.get() < 1 {
            if limit_value.get() <= 0 {
                Err(
                    super::PaginationStartsWithOneTryNewError::LimitIsLessThanOrEqToZero {
                        limit: limit_value,
                        location: location_macros::location!(),
                    },
                )
            } else {
                Err(
                    super::PaginationStartsWithOneTryNewError::OffsetIsLessThanOne {
                        offset: offset_value,
                        location: location_macros::location!(),
                    },
                )
            }
        } else if offset_value.get().checked_add(limit_value.get()).is_some() {
            Ok(Self::from(
                pg_crud_common::domain_types::PaginationBase::new_unchecked(
                    limit_value.get(),
                    offset_value.get(),
                ),
            ))
        } else {
            Err(
                super::PaginationStartsWithOneTryNewError::OffsetPlusLimitIsIntOverflow {
                    limit: limit_value,
                    offset: offset_value,
                    location: location_macros::location!(),
                },
            )
        }
    }
}

impl TryFrom<PaginationStartsWithOneRaw> for PaginationStartsWithOne {
    type Error = super::PaginationStartsWithOneTryNewError;
    fn try_from(v: PaginationStartsWithOneRaw) -> Result<Self, Self::Error> {
        Self::try_new(v.limit, v.offset)
    }
}

impl<'lt> pg_crud_common::domain_types::PgTypeWhereFilter<'lt> for PaginationStartsWithOne {
    fn query_bind(
        self,
        query: pg_crud_common::domain_types::SqlxPostgresQuery<'lt>,
    ) -> Result<
        pg_crud_common::domain_types::SqlxPostgresQuery<'lt>,
        pg_crud_common::domain_types::SqlxPostgresQueryBindError,
    > {
        self.0.query_bind(query)
    }
    fn query_part(
        &self,
        increment: &mut dyn pg_crud_common::domain_types::QueryPartIncrementMut,
        column: pg_crud_common::domain_types::SqlColumnRef<'_>,
        add_operator: pg_crud_common::domain_types::AddOperator,
    ) -> Result<
        pg_crud_common::domain_types::QueryPartFragment,
        pg_crud_common::domain_types::QueryPartError,
    > {
        self.0.query_part(increment, column, add_operator)
    }
}

impl pg_crud_common::domain_types::DefaultSomeOneElement for PaginationStartsWithOne {
    #[inline]
    fn default_some_one_element() -> Self {
        Self::from(pg_crud_common::domain_types::PaginationBase::new_unchecked(
            pg_crud_common::domain_types::PaginationPolicy::standard()
                .default_limit()
                .get(),
            1,
        ))
    }
}

impl pg_crud_common::domain_types::DefaultSomeOneElementMaxPageSize for PaginationStartsWithOne {
    #[inline]
    fn default_some_one_element_max_page_size() -> Self {
        let one: i32 = 1;
        Self::from(pg_crud_common::domain_types::PaginationBase::new_unchecked(
            i32::MAX - one,
            one,
        ))
    }
}
