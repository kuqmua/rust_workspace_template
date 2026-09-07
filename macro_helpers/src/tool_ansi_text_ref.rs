#[derive(
    Debug,
    Clone,
    Copy,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct ToolAnsiTextRef<'text>(#[getters(copy)] &'text str);
