#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DebugRedacted,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub struct AdminOpaqueToken(server_admin_core::secrecy_admin_string::SecrecyAdminString);
