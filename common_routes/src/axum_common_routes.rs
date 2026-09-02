#[derive(
    Debug,
    Clone,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::IntoInnerFrom,
    proc_macro_newtype::FromInner,
)]
pub struct AxumCommonRoutes(axum::Router);
