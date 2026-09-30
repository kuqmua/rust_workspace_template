#[proc_macro_location_errors_with_location::errors_with_location]
#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    proc_macro_location_derive_location::Location,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum PaginationStartsWithOneTryNewError {
    LimitIsLessThanOrEqToZero {
        #[error_field_to_err_string_serde]
        limit: crate::pagination_starts_with_one_value::PaginationStartsWithOneValue,
    },
    OffsetIsLessThanOne {
        #[error_field_to_err_string_serde]
        offset: crate::pagination_starts_with_one_value::PaginationStartsWithOneValue,
    },
    OffsetPlusLimitIsIntOverflow {
        #[error_field_to_err_string_serde]
        limit: crate::pagination_starts_with_one_value::PaginationStartsWithOneValue,
        #[error_field_to_err_string_serde]
        offset: crate::pagination_starts_with_one_value::PaginationStartsWithOneValue,
    },
}
