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
        limit: crate::domain_types::PaginationLimit,
    },
    OffsetIsLessThanZero {
        #[eo_to_err_string_serde]
        offset: crate::domain_types::PaginationOffset,
    },
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_err_string_serde]
        limit: crate::domain_types::PaginationLimit,
        #[eo_to_err_string_serde]
        offset: crate::domain_types::PaginationOffset,
    },
}
