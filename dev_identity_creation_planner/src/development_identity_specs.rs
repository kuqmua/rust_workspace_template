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
    validator = |value: &[server_runtime_http::domain_types::IdentitySpec<Login, DisplayName, Role, SecretSource>]| {
        if value.len() > super::development_identity_specs_max_len::DEVELOPMENT_IDENTITY_SPECS_MAX_LEN {
            Err(DevelopmentIdentitySpecsError)
        } else { Ok(()) }
    }
)]
pub struct DevelopmentIdentitySpecs<Login, DisplayName, Role, SecretSource>(
    Vec<server_runtime_http::domain_types::IdentitySpec<Login, DisplayName, Role, SecretSource>>,
);
