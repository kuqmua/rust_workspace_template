#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::IntoInnerFrom,
    newtype::FromInner,
    generate_accessor::Getters,
)]
#[getters(get_mut)]
pub struct AxumAdminResponse(axum::response::Response);
