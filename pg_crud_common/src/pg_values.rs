#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum EqOperator {
    Eq,
    IsNull,
}
impl EqOperator {
    #[must_use]
    pub fn to_query_str(&self) -> EqOperatorQueryStr {
        match &self {
            Self::Eq => EqOperatorQueryStr::from(constants_str::PG_CRUD_EQUALITY_SQL_OPERATOR),
            Self::IsNull => EqOperatorQueryStr::from(constants_str::IS_NULL),
        }
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefInner,
    newtype::Display,
    newtype::FromInner,
)]
pub struct EqOperatorQueryStr(&'static str);
pub trait PgTypeEqOperator {
    fn operator(&self) -> EqOperator;
}
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
)]
#[serde(try_from = "i32")]
pub struct UnsignedPartOfI32(i32);
impl From<u16> for UnsignedPartOfI32 {
    fn from(value: u16) -> Self {
        Self(i32::from(value))
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum UnsignedPartOfI32TryFromI32Error {
    LessThanZero {
        location: location_lib::domain_types::Location,
        #[eo_to_err_string_serde]
        v: UnsignedPartOfI32Raw,
    },
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::Display,
    newtype::FromInner,
    newtype::GetInner,
)]
#[serde(from = "i32")]
pub struct UnsignedPartOfI32Raw(i32);
impl to_err_string::domain_types::ToErrString for UnsignedPartOfI32Raw {
    fn to_err_string(&self) -> to_err_string::domain_types::ErrorText {
        to_err_string::domain_types::ErrorText::try_from(self.to_string())
            .unwrap_or_else(to_err_string::domain_types::ErrorText::from)
    }
}
impl TryFrom<i32> for UnsignedPartOfI32 {
    type Error = UnsignedPartOfI32TryFromI32Error;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        if v >= 0 {
            Ok(Self(v))
        } else {
            Err(Self::Error::LessThanZero {
                v: UnsignedPartOfI32Raw::from(v),
                location: location_macros::location!(),
            })
        }
    }
}
impl to_err_string::domain_types::ToErrString for UnsignedPartOfI32 {
    fn to_err_string(&self) -> to_err_string::domain_types::ErrorText {
        to_err_string::domain_types::ErrorText::try_from(self.0.to_string())
            .unwrap_or_else(to_err_string::domain_types::ErrorText::from)
    }
}
impl sqlx::Type<sqlx::Postgres> for UnsignedPartOfI32 {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <i32 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <i32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for UnsignedPartOfI32 {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        <i32 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.0, buf)
    }
}
impl UnsignedPartOfI32 {
    #[must_use]
    pub const fn get(&self) -> Self {
        *self
    }
}
impl super::DefaultSomeOneElement for UnsignedPartOfI32 {
    fn default_some_one_element() -> Self {
        Self::from(constants_u16::ZERO)
    }
}
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
)]
#[serde(try_from = "i32")]
pub struct NotZeroUnsignedPartOfI32(UnsignedPartOfI32);
impl From<std::num::NonZeroU16> for NotZeroUnsignedPartOfI32 {
    fn from(value: std::num::NonZeroU16) -> Self {
        Self(UnsignedPartOfI32::from(value.get()))
    }
}
impl utoipa::PartialSchema for NotZeroUnsignedPartOfI32 {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::Integer)
            .minimum(Some(1.0f64))
            .maximum(Some(f64::from(i32::MAX)))
            .into()
    }
}
impl utoipa::ToSchema for NotZeroUnsignedPartOfI32 {}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum NotZeroUnsignedPartOfI32TryFromI32Error {
    IsZero {
        location: location_lib::domain_types::Location,
    },
    UnsignedPartOfI32TryFromI32Error {
        #[eo_location]
        v: UnsignedPartOfI32TryFromI32Error,
        location: location_lib::domain_types::Location,
    },
}
impl TryFrom<i32> for NotZeroUnsignedPartOfI32 {
    type Error = NotZeroUnsignedPartOfI32TryFromI32Error;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        let v0 = UnsignedPartOfI32::try_from(v).map_err(|error| {
            Self::Error::UnsignedPartOfI32TryFromI32Error {
                v: error,
                location: location_macros::location!(),
            }
        })?;
        if v0.0 == 0 {
            Err(Self::Error::IsZero {
                location: location_macros::location!(),
            })
        } else {
            Ok(Self(v0))
        }
    }
}
impl to_err_string::domain_types::ToErrString for NotZeroUnsignedPartOfI32 {
    fn to_err_string(&self) -> to_err_string::domain_types::ErrorText {
        self.0.to_err_string()
    }
}
impl sqlx::Type<sqlx::Postgres> for NotZeroUnsignedPartOfI32 {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <UnsignedPartOfI32 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <UnsignedPartOfI32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for NotZeroUnsignedPartOfI32 {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        <UnsignedPartOfI32 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.0, buf)
    }
}
impl NotZeroUnsignedPartOfI32 {
    #[must_use]
    pub const fn get(&self) -> UnsignedPartOfI32 {
        self.0.get()
    }
}
impl super::DefaultSomeOneElement for NotZeroUnsignedPartOfI32 {
    fn default_some_one_element() -> Self {
        Self::from(std::num::NonZeroU16::MIN)
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum SingleOrMultiple<T: std::fmt::Debug + PartialEq + Clone> {
    Multiple(super::NotEmptyUniqueVec<T>),
    Single(T),
}
impl<T> utoipa::PartialSchema for SingleOrMultiple<T>
where
    T: std::fmt::Debug + PartialEq + Clone + utoipa::PartialSchema,
{
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::schema::Schema::from(
            utoipa::openapi::OneOfBuilder::new()
                .item(
                    utoipa::openapi::ObjectBuilder::new()
                        .property(
                            stringify!(Multiple),
                            <super::NotEmptyUniqueVec<T> as utoipa::PartialSchema>::schema(),
                        )
                        .required(stringify!(Multiple)),
                )
                .item(
                    utoipa::openapi::ObjectBuilder::new()
                        .property(stringify!(Single), <T as utoipa::PartialSchema>::schema())
                        .required(stringify!(Single)),
                )
                .build(),
        )
        .into()
    }
}
impl<T> utoipa::ToSchema for SingleOrMultiple<T>
where
    T: std::fmt::Debug + PartialEq + Clone + utoipa::ToSchema,
{
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(stringify!(SingleOrMultiple))
    }
    fn schemas(
        schemas: &mut Vec<(
            String,
            utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
        )>,
    ) {
        T::schemas(schemas);
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::IntoIterator,
)]
pub struct UuidUuidTestCases([uuid::Uuid; 1]);
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn i8_test_cases_vec() -> [i8; 3] {
    [i8::MIN, 0, i8::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn i16_test_cases_vec() -> [i16; 3] {
    [i16::MIN, 0, i16::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn i32_test_cases_vec() -> [i32; 3] {
    [i32::MIN, 0, i32::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn i64_test_cases_vec() -> [i64; 3] {
    [i64::MIN, 0, i64::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn u8_test_cases_vec() -> [u8; 3] {
    [u8::MIN, 0, u8::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn u16_test_cases_vec() -> [u16; 3] {
    [u16::MIN, 0, u16::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn u32_test_cases_vec() -> [u32; 3] {
    [u32::MIN, 0, u32::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn u64_test_cases_vec() -> [u64; 3] {
    [u64::MIN, 0, u64::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn f32_test_cases_vec() -> [f32; 18] {
    [
        f32::EPSILON,
        f32::MAX,
        f32::MIN,
        f32::MIN_POSITIVE,
        -1e30,
        -1e-30,
        -16_777_214.0,
        -100.0,
        -10.0,
        -1.0,
        -0.0,
        0.0,
        1.0,
        10.0,
        100.0,
        16_777_214.0,
        1e-30,
        1e30,
    ]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn f64_test_cases_vec() -> [f64; 18] {
    [
        f64::EPSILON,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        -1e300,
        -1e-300,
        -9_007_199_254_740_990.0,
        -100.0,
        -10.0,
        -1.0,
        -0.0,
        0.0,
        1.0,
        10.0,
        100.0,
        9_007_199_254_740_990.0,
        1e-300,
        1e300,
    ]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn bool_test_cases_vec() -> [bool; 2] {
    [true, false]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub fn string_test_cases_vec() -> [String; 12] {
    [
        String::new(),
        constants_str::A_ALT.to_owned(),
        constants_str::HELLO_WORLD.to_owned(),
        constants_str::THREE_SPACES.to_owned(),
        constants_str::NEWLINE_CARRIAGE_RETURN_TAB.to_owned(),
        constants_str::VALUE_1234567890.to_owned(),
        constants_str::U_1F600.to_owned(),
        constants_str::U_3053_U_3093_U_306B_U_3061_U_306F.to_owned(),
        constants_str::U_1F30D_U_1F680_U_2728_RUST_U_1F496_U_1F980.to_owned(),
        constants_str::A_ALT.repeat(1024),
        constants_str::LINE1_NEWLINE_LINE2_NEWLINE_LINE3.to_owned(),
        constants_str::U_1F496.to_owned(),
    ]
}
#[must_use]
pub fn uuid_uuid_test_cases_vec() -> UuidUuidTestCases {
    UuidUuidTestCases::from([uuid::Uuid::from_u128(
        0x123e_4567_e89b_42d3_a456_4266_1417_4000u128,
    )])
}
#[cfg(test)]
#[path = "domain_types_pg_values_tests.rs"]
mod tests;
