#[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub enum IsStandardNonNull {
    False,
    True,
}
