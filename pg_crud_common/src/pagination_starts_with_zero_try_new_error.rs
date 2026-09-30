#[proc_macro_location_errors_with_location::errors_with_location]
#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    proc_macro_location_derive_location::Location,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum PaginationStartsWithZeroTryNewError {
    LimitIsLessThanOrEqToZero {
        #[error_field_to_err_string_serde]
        limit: crate::pagination_limit::PaginationLimit,
    },
    OffsetIsLessThanZero {
        #[error_field_to_err_string_serde]
        offset: crate::pagination_offset::PaginationOffset,
    },
    OffsetPlusLimitIsIntOverflow {
        #[error_field_to_err_string_serde]
        limit: crate::pagination_limit::PaginationLimit,
        #[error_field_to_err_string_serde]
        offset: crate::pagination_offset::PaginationOffset,
    },
}
