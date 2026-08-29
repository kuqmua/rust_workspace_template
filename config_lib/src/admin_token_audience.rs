#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    serde::Deserialize,
    serde::Serialize,
    newtype::BoundedString,
    newtype::AsRefOwned,
)]
#[bounded_string(max = 256, description = "administrator token audience")]
#[serde(try_from = "String")]
pub struct AdminTokenAudience(String);

impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for AdminTokenAudience {
    type Error = crate::try_from_std_env_var_ok_admin_token_text_error::TryFromStdEnvVarOkAdminTokenTextError;

    fn try_from_std_env_var_ok(v: crate::std_env_var_ok::StdEnvVarOk) -> Result<Self, Self::Error> {
        super::parse_admin_token_text::parse_admin_token_text(v, Self::try_from)
    }
}
