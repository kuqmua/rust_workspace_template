#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::FromInner)]
#[allow(
    dead_code,
    reason = "the owned permit is held for its drop semantics while password hashing runs"
)]
#[derive(proc_macro_getters::Getters)]
pub(crate) struct TokioAdminOwnedSemaphorePermit(tokio::sync::OwnedSemaphorePermit);
