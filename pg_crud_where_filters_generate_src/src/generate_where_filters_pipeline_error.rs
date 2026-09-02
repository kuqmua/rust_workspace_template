#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum GenerateWhereFiltersPipelineError {
    #[error("{}", constants_str::INVALID_FILTER_SPECIFICATION)]
    InvalidContract,
    #[error("{0}")]
    Parse(crate::serde_json_generate_where_filters_error::SerdeJsonGenerateWhereFiltersError),
}
