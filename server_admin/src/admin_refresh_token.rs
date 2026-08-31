#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DebugRedacted,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub struct AdminRefreshToken(crate::admin_opaque_token::AdminOpaqueToken);
