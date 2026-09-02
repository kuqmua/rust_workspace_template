#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype::IntoInnerFrom,
    proc_macro_newtype::FromInner,
)]
pub struct AxumAdminFrontendRouter(axum::Router);
