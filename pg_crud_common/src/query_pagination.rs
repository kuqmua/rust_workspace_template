#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    utoipa::ToSchema,
    strum_macros::EnumString,
    optimal_memory_layout::OptimalMemoryLayout,
)]
#[strum(serialize_all = "snake_case")]
pub enum Order {
    #[serde(rename(serialize = "asc", deserialize = "asc"))]
    #[default]
    Asc,
    #[serde(rename(serialize = "desc", deserialize = "desc"))]
    Desc,
}
impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asc => write!(f, "{}", naming::domain_types::AscUpperCamelCase),
            Self::Desc => write!(f, "{}", naming::domain_types::DescUpperCamelCase),
        }
    }
}
impl super::DefaultSomeOneElement for Order {
    fn default_some_one_element() -> Self {
        Self::default()
    }
}
#[derive(
    Debug, Clone, PartialEq, Eq, optimal_memory_layout::OptimalMemoryLayout, newtype::Display,
)]
pub struct OrderSnakeCaseStr(String);
impl From<super::PgCrudStringWrapperTryFromStringError> for OrderSnakeCaseStr {
    fn from(value: super::PgCrudStringWrapperTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for OrderSnakeCaseStr {
    type Error = super::PgCrudStringWrapperTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > super::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: super::PG_CRUD_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
#[derive(
    Debug, Clone, PartialEq, Eq, optimal_memory_layout::OptimalMemoryLayout, newtype::Display,
)]
pub struct OrderUpperCamelCaseStr(String);
impl From<super::PgCrudStringWrapperTryFromStringError> for OrderUpperCamelCaseStr {
    fn from(value: super::PgCrudStringWrapperTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for OrderUpperCamelCaseStr {
    type Error = super::PgCrudStringWrapperTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > super::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: super::PG_CRUD_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
impl Order {
    #[must_use]
    pub fn to_snake_case_str(&self) -> OrderSnakeCaseStr {
        OrderSnakeCaseStr::try_from(naming_common::domain_types::DisplayToSnakeCaseStr::case(
            self,
        ))
        .unwrap_or_else(OrderSnakeCaseStr::from)
    }
    #[must_use]
    pub fn to_upper_camel_case_str(&self) -> OrderUpperCamelCaseStr {
        OrderUpperCamelCaseStr::try_from(
            naming_common::domain_types::DisplayToUpperCamelCaseStr::case(self),
        )
        .unwrap_or_else(OrderUpperCamelCaseStr::from)
    }
}
#[derive(
    Debug, serde::Serialize, serde::Deserialize, optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct OrderBy<ColumnGeneric> {
    pub column: ColumnGeneric,
    pub order: Option<Order>,
}
impl<ColumnGeneric: utoipa::PartialSchema> utoipa::__dev::ComposeSchema for OrderBy<ColumnGeneric> {
    fn compose(
        _new_generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                constants_str::COLUMN,
                <ColumnGeneric as utoipa::PartialSchema>::schema(),
            )
            .property(
                constants_str::ORDER,
                <Order as utoipa::PartialSchema>::schema(),
            )
            .required(constants_str::COLUMN)
            .build()
            .into()
    }
}
impl<ColumnGeneric: utoipa::ToSchema> utoipa::ToSchema for OrderBy<ColumnGeneric> {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(constants_str::ORDERBY)
    }
}
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
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct PaginationBase {
    limit: super::PaginationLimit,
    offset: super::PaginationOffset,
}
impl PaginationBase {
    #[must_use]
    pub fn end(&self) -> super::PaginationEnd {
        super::PaginationEnd::from(self.offset.get().saturating_add(self.limit.get()))
    }
    #[must_use]
    pub fn new_unchecked<LimitTy, OffsetTy>(limit: LimitTy, offset: OffsetTy) -> Self
    where
        LimitTy: Into<super::PaginationLimit>,
        OffsetTy: Into<super::PaginationOffset>,
    {
        Self {
            limit: limit.into(),
            offset: offset.into(),
        }
    }
    #[must_use]
    pub fn start(&self) -> super::PaginationStart {
        super::PaginationStart::from(self.offset.get())
    }
}
impl<'query_lt> super::PgTypeWhereFilter<'query_lt> for PaginationBase {
    fn query_bind(
        self,
        mut query: super::SqlxPostgresQuery<'query_lt>,
    ) -> Result<super::SqlxPostgresQuery<'query_lt>, super::SqlxPostgresQueryBindError> {
        if let Err(error) = query.as_mut().try_bind(self.limit.get()) {
            return Err(super::SqlxPostgresQueryBindError::from(error));
        }
        if let Err(error) = query.as_mut().try_bind(self.offset.get()) {
            return Err(super::SqlxPostgresQueryBindError::from(error));
        }
        Ok(query)
    }
    fn query_part(
        &self,
        increment: &mut dyn super::QueryPartIncrementMut,
        _: super::SqlColumnRef<'_>,
        _: super::AddOperator,
    ) -> Result<super::QueryPartFragment, super::QueryPartError> {
        let limit_increment = match super::increment_checked_add_one_returning_increment(increment)
        {
            Ok(v) => v,
            Err(error) => {
                return Err(error);
            }
        };
        let offset_increment = match super::increment_checked_add_one_returning_increment(increment)
        {
            Ok(v) => v,
            Err(error) => {
                return Err(error);
            }
        };
        let mut query_part = String::with_capacity(32);
        if std::fmt::Write::write_fmt(
            &mut query_part,
            format_args!("limit ${limit_increment} offset ${offset_increment}"),
        )
        .is_err()
        {
            return Err(super::QueryPartError::WriteIntoBuffer {
                location: location_macros::location!(),
            });
        }
        Ok(super::QueryPartFragment::try_from(query_part)?)
    }
}
impl Default for PaginationBase {
    fn default() -> Self {
        Self::new_unchecked(super::PaginationPolicy::standard().default_limit().get(), 0)
    }
}
#[derive(
    Debug, serde::Deserialize, schemars::JsonSchema, optimal_memory_layout::OptimalMemoryLayout,
)]
struct PaginationStartsWithZeroRaw {
    limit: super::PaginationLimit,
    offset: super::PaginationOffset,
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
#[serde(try_from = "PaginationStartsWithZeroRaw")]
#[derive(newtype::FromInner)]
pub struct PaginationStartsWithZero(PaginationBase);

#[location::errors_with_location]
#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum PaginationStartsWithZeroTryNewError {
    LimitIsLessThanOrEqToZero {
        #[eo_to_err_string_serde]
        limit: super::PaginationLimit,
    },
    OffsetIsLessThanZero {
        #[eo_to_err_string_serde]
        offset: super::PaginationOffset,
    },
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_err_string_serde]
        limit: super::PaginationLimit,
        #[eo_to_err_string_serde]
        offset: super::PaginationOffset,
    },
}
impl PaginationStartsWithZero {
    #[must_use]
    pub fn end(&self) -> super::PaginationEnd {
        self.0.end()
    }
    #[must_use]
    pub fn start(&self) -> super::PaginationStart {
        self.0.start()
    }
    pub fn try_new<LimitTy, OffsetTy>(
        limit: LimitTy,
        offset: OffsetTy,
    ) -> Result<Self, PaginationStartsWithZeroTryNewError>
    where
        LimitTy: Into<super::PaginationLimit>,
        OffsetTy: Into<super::PaginationOffset>,
    {
        let limit_value = limit.into();
        let offset_value = offset.into();
        if limit_value.get() <= 0 || offset_value.get() < 0 {
            if limit_value.get() <= 0 {
                Err(
                    PaginationStartsWithZeroTryNewError::LimitIsLessThanOrEqToZero {
                        limit: limit_value,
                        location: location_macros::location!(),
                    },
                )
            } else {
                Err(PaginationStartsWithZeroTryNewError::OffsetIsLessThanZero {
                    offset: offset_value,
                    location: location_macros::location!(),
                })
            }
        } else if offset_value.get().checked_add(limit_value.get()).is_some() {
            Ok(Self::from(PaginationBase::new_unchecked(
                limit_value,
                offset_value,
            )))
        } else {
            Err(
                PaginationStartsWithZeroTryNewError::OffsetPlusLimitIsIntOverflow {
                    limit: limit_value,
                    offset: offset_value,
                    location: location_macros::location!(),
                },
            )
        }
    }
}
impl TryFrom<PaginationStartsWithZeroRaw> for PaginationStartsWithZero {
    type Error = PaginationStartsWithZeroTryNewError;
    fn try_from(v: PaginationStartsWithZeroRaw) -> Result<Self, Self::Error> {
        Self::try_new(v.limit, v.offset)
    }
}
impl<'query_lt> super::PgTypeWhereFilter<'query_lt> for PaginationStartsWithZero {
    fn query_bind(
        self,
        query: super::SqlxPostgresQuery<'query_lt>,
    ) -> Result<super::SqlxPostgresQuery<'query_lt>, super::SqlxPostgresQueryBindError> {
        self.0.query_bind(query)
    }
    fn query_part(
        &self,
        increment: &mut dyn super::QueryPartIncrementMut,
        column: super::SqlColumnRef<'_>,
        add_operator: super::AddOperator,
    ) -> Result<super::QueryPartFragment, super::QueryPartError> {
        self.0.query_part(increment, column, add_operator)
    }
}
impl super::DefaultSomeOneElement for PaginationStartsWithZero {
    #[inline]
    fn default_some_one_element() -> Self {
        Self::from(PaginationBase::new_unchecked(
            super::PaginationPolicy::standard().default_limit().get(),
            0,
        ))
    }
}
impl super::DefaultSomeOneElementMaxPageSize for PaginationStartsWithZero {
    #[inline]
    fn default_some_one_element_max_page_size() -> Self {
        Self::from(PaginationBase::new_unchecked(i32::MAX, 0))
    }
}
#[cfg(test)]
#[path = "domain_types_query_pagination_tests.rs"]
mod tests;
