#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    generate_constructor::New,
)]
pub struct DevelopmentIdentityCreationPlan<Login, DisplayName, Role, SecretSource> {
    identities: crate::development_identity_specs::DevelopmentIdentitySpecs<
        Login,
        DisplayName,
        Role,
        SecretSource,
    >,
}

impl<Login, DisplayName, Role, SecretSource>
    DevelopmentIdentityCreationPlan<Login, DisplayName, Role, SecretSource>
{
    #[must_use]
    pub fn identities(
        &self,
    ) -> &[server_runtime_core::identity_spec::IdentitySpec<Login, DisplayName, Role, SecretSource>]
    {
        self.identities.as_ref()
    }
}
