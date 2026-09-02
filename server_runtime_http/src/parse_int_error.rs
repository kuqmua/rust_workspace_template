#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum ParseIntError {
    #[error("{0}")]
    Parse(#[from] std::num::ParseIntError),
}
