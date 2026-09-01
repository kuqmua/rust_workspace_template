#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct PaginationPolicy {
    #[getters(copy)]
    default_limit: super::pagination_limit::PaginationLimit,
}
impl PaginationPolicy {
    #[must_use]
    pub fn standard() -> Self {
        Self {
            default_limit: super::pagination_limit::PaginationLimit::from(5i64),
        }
    }
}

#[cfg(test)]
mod test_policy_tests {
    #[test]
    fn test_default_limit_is_owned_by_typed_policy() {
        assert_eq!(
            crate::pagination_policy::PaginationPolicy::standard()
                .default_limit()
                .get(),
            5i64
        );
    }
}
