#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub(crate) struct AxumApiRoutes(axum::Router);
