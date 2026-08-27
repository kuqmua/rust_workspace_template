#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct AxumMethodFilter(axum::routing::MethodFilter);
