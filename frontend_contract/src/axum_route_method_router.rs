#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, proc_macro_newtype::FromInner, proc_macro_newtype::IntoInnerFrom)]
pub struct AxumRouteMethodRouter<State>(axum::routing::MethodRouter<State>);
