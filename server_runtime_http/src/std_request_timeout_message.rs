#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, serde::Serialize)]
#[serde(transparent)]
#[derive(newtype::FromInner)]
pub(super) struct StdRequestTimeoutMessage(&'static str);
