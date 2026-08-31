#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::FromInner,
    newtype::DerefInner,
    newtype::IntoInner,
    generate_accessor::Getters,
)]
pub(crate) struct AxumAdminPath<Value>(Value);
