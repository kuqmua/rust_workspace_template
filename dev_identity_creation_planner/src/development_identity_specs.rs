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
    error = crate::development_identity_specs_error::DevelopmentIdentitySpecsError,
    validator = |value: &[server_runtime_core::identity_spec::IdentitySpec<Login, DisplayName, Role, SecretSource>]| {
        if value.len() > crate::development_identity_specs_max_len::DEVELOPMENT_IDENTITY_SPECS_MAX_LEN {
            Err(crate::development_identity_specs_error::DevelopmentIdentitySpecsError)
        } else { Ok(()) }
    }
)]
pub struct DevelopmentIdentitySpecs<Login, DisplayName, Role, SecretSource>(
    Vec<server_runtime_core::identity_spec::IdentitySpec<Login, DisplayName, Role, SecretSource>>,
);
