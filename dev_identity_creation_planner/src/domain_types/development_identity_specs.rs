use super::DevelopmentIdentitySpecsError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefTarget,
    newtype::TryFrom,
)]
#[try_from(
    validator = DevelopmentIdentitySpecs::<Login, DisplayName, Role, SecretSource>::validate
)]
pub struct DevelopmentIdentitySpecs<Login, DisplayName, Role, SecretSource>(
    Vec<server_runtime_http::domain_types::IdentitySpec<Login, DisplayName, Role, SecretSource>>,
);

impl<Login, DisplayName, Role, SecretSource>
    DevelopmentIdentitySpecs<Login, DisplayName, Role, SecretSource>
{
    #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call
    const fn validate(
        value: &[server_runtime_http::domain_types::IdentitySpec<
            Login,
            DisplayName,
            Role,
            SecretSource,
        >],
    ) -> Result<(), DevelopmentIdentitySpecsError> {
        if value.len()
            > super::development_identity_specs_max_len::DEVELOPMENT_IDENTITY_SPECS_MAX_LEN
        {
            Err(DevelopmentIdentitySpecsError)
        } else {
            Ok(())
        }
    }
}
