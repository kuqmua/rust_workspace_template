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
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::Display,
    newtype::FromInner,
    newtype::GetInner,
    newtype::ToErrString,
)]
#[serde(from = "i64")]
pub struct PaginationLimit(i64);
impl From<i32> for PaginationLimit {
    fn from(value: i32) -> Self {
        Self(value.into())
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
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
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::Display,
    newtype::FromInner,
    newtype::GetInner,
    newtype::ToErrString,
)]
#[serde(from = "i64")]
pub struct PaginationOffset(i64);
impl From<i32> for PaginationOffset {
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
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct PaginationStart(i64);
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct PaginationEnd(i64);
