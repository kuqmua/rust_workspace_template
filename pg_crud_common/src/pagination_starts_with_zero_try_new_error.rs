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
        #[eo_to_err_string_serde]
        limit: crate::pagination_limit::PaginationLimit,
    },
    OffsetIsLessThanZero {
        #[eo_to_err_string_serde]
        offset: crate::pagination_offset::PaginationOffset,
    },
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_err_string_serde]
        limit: crate::pagination_limit::PaginationLimit,
        #[eo_to_err_string_serde]
        offset: crate::pagination_offset::PaginationOffset,
    },
}
