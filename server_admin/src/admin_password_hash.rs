#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_debug_redacted::DebugRedacted,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
)]
pub struct AdminPasswordHash(pg_types_text_misc::generate_pg_types_mod::StringAsNonNullTextSecret);

impl AdminPasswordHash {
    #[must_use]
    pub(crate) fn expose(&self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'_> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(self.get_inner().as_ref())
    }

    #[must_use]
    pub fn new(
        string_as_non_null_text_secret: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullTextSecret,
    ) -> Self {
        Self::from(string_as_non_null_text_secret)
    }
}
