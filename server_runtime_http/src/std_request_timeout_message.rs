#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, serde::Serialize,
)]
#[serde(transparent)]
#[derive(proc_macro_newtype::FromInner)]
pub(super) struct StdRequestTimeoutMessage(&'static str);
