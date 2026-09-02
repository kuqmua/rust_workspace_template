#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum BoundedJsonReadError {
    #[error("bounded content read failed")]
    Read(#[source] crate::bounded_read_error::BoundedReadError),
    #[error("bounded content is not valid JSON")]
    SerdeJson(#[source] crate::serde_json_error::SerdeJsonError),
}
