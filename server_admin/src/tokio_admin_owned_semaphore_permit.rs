#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
#[allow(
    dead_code,
    reason = "the owned permit is held for its drop semantics while password hashing runs"
)]
#[derive(generate_accessor::Getters)]
pub(crate) struct TokioAdminOwnedSemaphorePermit(tokio::sync::OwnedSemaphorePermit);
