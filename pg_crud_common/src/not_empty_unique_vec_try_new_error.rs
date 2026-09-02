#[proc_macro_location::errors_with_location]
#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    proc_macro_location::Location,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum NotEmptyUniqueVecTryNewError<T> {
    IsEmpty {},
    NotUnique {
        #[eo_to_err_string_serde]
        v: T,
    },
    TooLong {},
}
