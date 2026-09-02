#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::DebugRedacted,
    proc_macro_newtype::FromInner,
    proc_macro_getters::Getters,
)]
pub struct AdminTokenHash(server_admin_core::secrecy_admin_string::SecrecyAdminString);
