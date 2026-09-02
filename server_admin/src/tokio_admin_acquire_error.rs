#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::DebugTransparent,
    proc_macro_newtype::FromInner,
    proc_macro_getters::Getters,
)]
pub struct TokioAdminAcquireError(tokio::sync::AcquireError);
