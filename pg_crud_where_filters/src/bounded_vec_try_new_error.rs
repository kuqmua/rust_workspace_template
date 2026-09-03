#[proc_macro_location_errors_with_location::errors_with_location]
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    proc_macro_location_derive_location::Location,
    schemars::JsonSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum BoundedVecTryNewError {
    LenIsNotCorrect {
        #[eo_to_err_string_serde]
        wrong_len: crate::pg_filter_vec_len::PgFilterVecLen,
        #[eo_to_err_string_serde]
        expected: crate::pg_filter_vec_len::PgFilterVecLen,
    },
}
