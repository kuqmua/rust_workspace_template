#[derive(Debug, serde::Deserialize, schemars::JsonSchema, optml::Optml)]
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
    optml::Optml,
    newtype::Newtype,
)]
#[newtype(display, from, to_err_string)]
pub struct PaginationStartsWithOneValue(i64);
impl PaginationStartsWithOneValue {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, optml::Optml, newtype::Newtype)]
#[newtype(from_inner, into_inner_from)]
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
    optml::Optml,
)]
#[serde(try_from = "PaginationStartsWithOneRaw")]
pub struct PaginationStartsWithOne(pg_crud_common::PaginationBase);
#[location::errors_with_location]
#[derive(
    Debug, serde::Serialize, serde::Deserialize, thiserror::Error, location::Location, optml::Optml,
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
            Ok(Self(pg_crud_common::PaginationBase::new_unchecked(
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
        Self(pg_crud_common::PaginationBase::new_unchecked(
            pg_crud_common::DEFAULT_PAGINATION_LIMIT,
            1,
        ))
    }
}
impl pg_crud_common::DefaultSomeOneElementMaxPageSize for PaginationStartsWithOne {
    #[inline]
    fn default_some_one_element_max_page_size() -> Self {
        let one: i32 = 1;
        Self(pg_crud_common::PaginationBase::new_unchecked(
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
        str_constants::PRIMARY_KEY
    } else {
        str_constants::PG_CRUD_EMPTY_SQL_SUFFIX
    }
}
