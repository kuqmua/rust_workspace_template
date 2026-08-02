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
#[serde(from = "i64")]
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
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PaginationPolicy {
    default_limit: PaginationLimit,
}
impl PaginationPolicy {
    #[must_use]
    pub const fn default_limit(self) -> PaginationLimit {
        self.default_limit
    }
    #[must_use]
    pub fn standard() -> Self {
        Self {
            default_limit: PaginationLimit::from(5i64),
        }
    }
}

#[cfg(test)]
mod policy_tests {
    #[test]
    fn default_limit_is_owned_by_typed_policy() {
        assert_eq!(
            super::PaginationPolicy::standard().default_limit().get(),
            5i64
        );
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
#[serde(from = "i64")]
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
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, optml::Optml, newtype::FromInner,
)]
pub struct PaginationStart(i64);
impl PaginationStart {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, optml::Optml, newtype::FromInner,
)]
pub struct PaginationEnd(i64);
impl PaginationEnd {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
