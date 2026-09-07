#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_debug_redacted::DebugRedacted,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
)]
pub struct RuntimeAdminJwtSecret(server_admin_core::secrecy_admin_string::SecrecyAdminString);

impl RuntimeAdminJwtSecret {
    #[must_use]
    pub fn new(
        secrecy_admin_string: server_admin_core::secrecy_admin_string::SecrecyAdminString,
    ) -> Self {
        Self::from(secrecy_admin_string)
    }
}
