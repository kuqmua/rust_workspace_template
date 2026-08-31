#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::IntoInnerFrom,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub struct AxumAdminAuthRouter(axum::Router);
