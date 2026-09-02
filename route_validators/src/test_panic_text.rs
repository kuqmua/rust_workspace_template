#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::Display,
    proc_macro_newtype::FromInner,
)]
pub(super) struct TestPanicText(&'static str);
