#[proc_macro_location::errors_with_location]
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    proc_macro_location::Location,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum BetweenTryNewError<T> {
    StartMoreOrEqToEnd {
        #[eo_to_err_string_serde]
        start: T,
        #[eo_to_err_string_serde]
        end: T,
    },
}
