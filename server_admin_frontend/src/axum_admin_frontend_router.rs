#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct AxumAdminFrontendRouter(axum::Router);
