#[proc_macro_location_errors_with_location::errors_with_location]
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    proc_macro_location_derive_location::Location,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum BetweenTryNewError<T> {
    StartNotLessThanOrEqualToEnd {
        #[error_field_to_err_string_serde]
        start: T,
        #[error_field_to_err_string_serde]
        end: T,
    },
}
