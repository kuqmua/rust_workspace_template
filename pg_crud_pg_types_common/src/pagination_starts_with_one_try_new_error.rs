#[location::errors_with_location]
#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum PaginationStartsWithOneTryNewError {
    LimitIsLessThanOrEqToZero {
        #[eo_to_err_string_serde]
        limit: crate::domain_types::PaginationStartsWithOneValue,
    },
    OffsetIsLessThanOne {
        #[eo_to_err_string_serde]
        offset: crate::domain_types::PaginationStartsWithOneValue,
    },
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_err_string_serde]
        limit: crate::domain_types::PaginationStartsWithOneValue,
        #[eo_to_err_string_serde]
        offset: crate::domain_types::PaginationStartsWithOneValue,
    },
}
