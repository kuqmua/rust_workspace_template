#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_debug_redacted::DebugRedacted,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
)]
pub struct AdminRefreshToken(crate::admin_opaque_token::AdminOpaqueToken);

impl AdminRefreshToken {
    #[must_use]
    pub fn new(admin_opaque_token: crate::admin_opaque_token::AdminOpaqueToken) -> Self {
        Self::from(admin_opaque_token)
    }
    #[must_use]
    pub fn expose(&self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'_> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            secrecy::ExposeSecret::expose_secret(self.get_inner().get_inner().as_ref()).as_str(),
        )
    }
}
