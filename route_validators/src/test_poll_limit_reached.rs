#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
    proc_macro_newtype::NotInner,
)]
pub(super) struct TestPollLimitReached(bool);
