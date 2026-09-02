#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub enum OptionalJsonContentType {
    ApplicationJson,
    Missing,
    NonJson,
}
