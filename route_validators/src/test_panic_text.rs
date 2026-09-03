#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct TestPanicText(&'static str);
