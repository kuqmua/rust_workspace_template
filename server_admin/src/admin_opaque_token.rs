#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_debug_redacted::DebugRedacted,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
)]
pub struct AdminOpaqueToken(server_admin_core::secrecy_admin_string::SecrecyAdminString);

impl AdminOpaqueToken {
    #[must_use]
    pub fn new(
        secrecy_admin_string: server_admin_core::secrecy_admin_string::SecrecyAdminString,
    ) -> Self {
        Self::from(secrecy_admin_string)
    }
    #[must_use]
    pub(crate) fn expose(&self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'_> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            secrecy::ExposeSecret::expose_secret(self.get_inner().as_ref()).as_str(),
        )
    }
    pub(crate) fn clone_secret(
        &self,
    ) -> server_admin_core::secrecy_admin_string::SecrecyAdminString {
        server_admin_core::secrecy_admin_string::SecrecyAdminString::from(secrecy::SecretBox::new(
            Box::new(secrecy::ExposeSecret::expose_secret(self.get_inner()).clone()),
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_opaque_token_debug_is_redacted() {
        let token = crate::admin_opaque_token::AdminOpaqueToken::new(
            server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(
                constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES.to_owned(),
            )
            .expect(constants_str::DIAGNOSTIC_4F0DB163),
        );
        let debug = format!("{token:?}");
        assert!(debug.contains(constants_str::REDACTED_ALT_3));
        assert!(!debug.contains(constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES));
    }
}
