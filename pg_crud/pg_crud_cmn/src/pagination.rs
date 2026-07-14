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
    newtype::Newtype,
)]
#[newtype(display, from, to_err_string)]
pub struct PgnLimit(i64);
impl PgnLimit {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
impl From<i32> for PgnLimit {
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
    newtype::Newtype,
)]
#[newtype(display, from, to_err_string)]
pub struct PgnOffset(i64);
impl PgnOffset {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
impl From<i32> for PgnOffset {
    fn from(value: i32) -> Self {
        Self(value.into())
    }
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, optml::Optml)]
pub struct PgnStart(i64);
impl From<i64> for PgnStart {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl PgnStart {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, optml::Optml)]
pub struct PgnEnd(i64);
impl From<i64> for PgnEnd {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl PgnEnd {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
