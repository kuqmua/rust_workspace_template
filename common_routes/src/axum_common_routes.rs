#[derive(
    Debug,
    Clone,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct AxumCommonRoutes(axum::Router);
