#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DevelopmentIdentityCreationPlan<Login, DisplayName, Role, SecretSource> {
    identities: super::DevelopmentIdentitySpecs<Login, DisplayName, Role, SecretSource>,
}

impl<Login, DisplayName, Role, SecretSource>
    DevelopmentIdentityCreationPlan<Login, DisplayName, Role, SecretSource>
{
    #[must_use]
    pub fn identities(
        &self,
    ) -> &[server_runtime_http::domain_types::IdentitySpec<Login, DisplayName, Role, SecretSource>]
    {
        self.identities.as_ref()
    }

    #[must_use]
    pub const fn new(
        identities: super::DevelopmentIdentitySpecs<Login, DisplayName, Role, SecretSource>,
    ) -> Self {
        Self { identities }
    }
}
