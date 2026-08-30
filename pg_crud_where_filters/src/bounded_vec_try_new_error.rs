#[location::errors_with_location]
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum BoundedVecTryNewError {
    LenIsNotCorrect {
        #[eo_to_err_string_serde]
        wrong_len: crate::pg_filter_vec_len::PgFilterVecLen,
        #[eo_to_err_string_serde]
        expected: crate::pg_filter_vec_len::PgFilterVecLen,
    },
}
