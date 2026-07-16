pub const DEFAULT_PAGINATION_LIMIT: i64 = 5;
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
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
    newtype::Display,
    newtype::FromInner,
    newtype::ToErrString,
)]
pub struct PaginationLimit(i64);
impl PaginationLimit {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
impl From<i32> for PaginationLimit {
    fn from(value: i32) -> Self {
        Self(value.into())
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
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
    newtype::Display,
    newtype::FromInner,
    newtype::ToErrString,
)]
pub struct PaginationOffset(i64);
impl PaginationOffset {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
impl From<i32> for PaginationOffset {
    fn from(value: i32) -> Self {
        Self(value.into())
    }
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, optml::Optml)]
pub struct PaginationStart(i64);
impl From<i64> for PaginationStart {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl PaginationStart {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, optml::Optml)]
pub struct PaginationEnd(i64);
impl From<i64> for PaginationEnd {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl PaginationEnd {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
