#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(super) struct TokioManagedChild(pub(super) tokio::process::Child);
