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
        limit: crate::pagination_starts_with_one_value::PaginationStartsWithOneValue,
    },
    OffsetIsLessThanOne {
        #[eo_to_err_string_serde]
        offset: crate::pagination_starts_with_one_value::PaginationStartsWithOneValue,
    },
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_err_string_serde]
        limit: crate::pagination_starts_with_one_value::PaginationStartsWithOneValue,
        #[eo_to_err_string_serde]
        offset: crate::pagination_starts_with_one_value::PaginationStartsWithOneValue,
    },
}
