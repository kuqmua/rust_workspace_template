#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_debug_redacted::DebugRedacted,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
)]
pub struct AdminTokenHash(server_admin_core::secrecy_admin_string::SecrecyAdminString);

impl AdminTokenHash {
    #[must_use]
    pub(crate) fn new(
        secrecy_admin_string: server_admin_core::secrecy_admin_string::SecrecyAdminString,
    ) -> Self {
        Self::from(secrecy_admin_string)
    }
    #[must_use]
    pub fn expose(&self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'_> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            secrecy::ExposeSecret::expose_secret(self.get_inner().as_ref()).as_str(),
        )
    }
}
