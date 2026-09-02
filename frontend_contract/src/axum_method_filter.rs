#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, proc_macro_newtype::FromInner, proc_macro_newtype::IntoInnerFrom)]
pub struct AxumMethodFilter(axum::routing::MethodFilter);
