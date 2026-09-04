#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_target::AsRefTarget,
)]
pub struct DevelopmentIdentitySpecs<Login, DisplayName, Role, SecretSource>(
    Vec<server_runtime_core::identity_spec::IdentitySpec<Login, DisplayName, Role, SecretSource>>,
);
impl<Login, DisplayName, Role, SecretSource>
    TryFrom<
        Vec<
            server_runtime_core::identity_spec::IdentitySpec<
                Login,
                DisplayName,
                Role,
                SecretSource,
            >,
        >,
    > for DevelopmentIdentitySpecs<Login, DisplayName, Role, SecretSource>
{
    type Error = crate::development_identity_specs_error::DevelopmentIdentitySpecsError;
    fn try_from(
        value: Vec<
            server_runtime_core::identity_spec::IdentitySpec<
                Login,
                DisplayName,
                Role,
                SecretSource,
            >,
        >,
    ) -> Result<Self, Self::Error> {
        if value.len()
            > crate::development_identity_specs_max_len::DEVELOPMENT_IDENTITY_SPECS_MAX_LEN
        {
            Err(Self::Error::TooMany)
        } else {
            Ok(Self(value))
        }
    }
}
