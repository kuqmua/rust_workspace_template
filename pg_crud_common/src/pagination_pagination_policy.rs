#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PaginationPolicy {
    default_limit: super::pagination_limit::PaginationLimit,
}
impl PaginationPolicy {
    #[must_use]
    pub const fn default_limit(self) -> super::pagination_limit::PaginationLimit {
        self.default_limit
    }
    #[must_use]
    pub fn standard() -> Self {
        Self {
            default_limit: super::pagination_limit::PaginationLimit::from(5i64),
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
