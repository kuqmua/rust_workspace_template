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
#[bounded_string(max = 256, description = "administrator token issuer")]
#[serde(try_from = "String")]
pub struct AdminTokenIssuer(String);

impl super::super::TryFromStdEnvVarOk for AdminTokenIssuer {
    type Error = super::TryFromStdEnvVarOkAdminTokenTextError;

    fn try_from_std_env_var_ok(v: super::super::StdEnvVarOk) -> Result<Self, Self::Error> {
        super::parse_admin_token_text::parse_admin_token_text(v, Self::try_from)
    }
}
