#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    proc_macro_generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    serde::Deserialize,
    serde::Serialize,
    proc_macro_newtype::BoundedStringWrapper,
    proc_macro_newtype::AsRefOwned,
)]
#[bounded_string(max = 256, description = "administrator token issuer")]
#[serde(try_from = "String")]
pub struct AdminTokenIssuer(bounded_types::bounded_string::BoundedString<0usize, 256, false>);

impl AdminTokenIssuer {
    #[must_use]
    pub const fn as_bounded_string(
        &self,
    ) -> &bounded_types::bounded_string::BoundedString<0usize, 256, false> {
        &self.0
    }
}

impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for AdminTokenIssuer {
    type Error = crate::try_from_std_env_var_ok_admin_token_text_error::TryFromStdEnvVarOkAdminTokenTextError;

    fn try_from_std_env_var_ok(
        std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk,
    ) -> Result<Self, Self::Error> {
        super::parse_admin_token_text::parse_admin_token_text(std_env_var_ok, Self::try_from)
    }
}
