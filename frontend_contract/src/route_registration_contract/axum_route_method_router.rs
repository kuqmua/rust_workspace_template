#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct AxumRouteMethodRouter<State>(axum::routing::MethodRouter<State>);
