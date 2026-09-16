#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminWhereManyTryFromStringError {
    #[error("invalid administrator where-many JSON")]
    Json(#[source] serde_json::Error),
    #[error("administrator where-many JSON must be an object")]
    NotObject,
    #[error("administrator where-many JSON length is outside the supported range")]
    Length(#[source] bounded_types::bounded_string_error::BoundedStringError),
}
