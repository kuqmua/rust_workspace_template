const DEVELOPMENT_IDENTITY_SPECS_MAX_LEN: usize = 1_024usize;

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
    Vec<server_runtime_http::IdentitySpec<Login, DisplayName, Role, SecretSource>>,
);

impl<Login, DisplayName, Role, SecretSource>
    DevelopmentIdentitySpecs<Login, DisplayName, Role, SecretSource>
{
    #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call
    const fn validate(
        value: &[server_runtime_http::IdentitySpec<Login, DisplayName, Role, SecretSource>],
    ) -> Result<(), DevelopmentIdentitySpecsError> {
        if value.len() > DEVELOPMENT_IDENTITY_SPECS_MAX_LEN {
            Err(DevelopmentIdentitySpecsError)
        } else {
            Ok(())
        }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{self:?}")]
pub struct DevelopmentIdentitySpecsError;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DevelopmentBootstrapPlan<Login, DisplayName, Role, SecretSource> {
    identities: DevelopmentIdentitySpecs<Login, DisplayName, Role, SecretSource>,
}

impl<Login, DisplayName, Role, SecretSource>
    DevelopmentBootstrapPlan<Login, DisplayName, Role, SecretSource>
{
    #[must_use]
    pub fn identities(
        &self,
    ) -> &[server_runtime_http::IdentitySpec<Login, DisplayName, Role, SecretSource>] {
        self.identities.as_ref()
    }

    #[must_use]
    pub const fn new(
        identities: DevelopmentIdentitySpecs<Login, DisplayName, Role, SecretSource>,
    ) -> Self {
        Self { identities }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Default, Eq, PartialEq,
)]
pub struct DevelopmentBootstrapSummary {
    already_exists: DevelopmentIdentityCount,
    create: DevelopmentIdentityCount,
    missing_role: DevelopmentIdentityCount,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct DevelopmentIdentityCount(usize);

impl DevelopmentBootstrapSummary {
    #[must_use]
    pub const fn already_exists(self) -> DevelopmentIdentityCount {
        self.already_exists
    }

    #[must_use]
    pub const fn create(self) -> DevelopmentIdentityCount {
        self.create
    }

    #[must_use]
    pub const fn missing_role(self) -> DevelopmentIdentityCount {
        self.missing_role
    }

    pub(crate) const fn record(
        &mut self,
        decision: server_runtime_http::IdentityBootstrapDecision,
    ) {
        match decision {
            server_runtime_http::IdentityBootstrapDecision::AlreadyExists => {
                self.already_exists.0 = self.already_exists.0.saturating_add(constants_usize::ONE);
            }
            server_runtime_http::IdentityBootstrapDecision::Create => {
                self.create.0 = self.create.0.saturating_add(constants_usize::ONE);
            }
            server_runtime_http::IdentityBootstrapDecision::MissingRole => {
                self.missing_role.0 = self.missing_role.0.saturating_add(constants_usize::ONE);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn plan_preserves_typed_identity_specs() {
        let plan = super::DevelopmentBootstrapPlan::new(
            super::DevelopmentIdentitySpecs::try_from(vec![
                server_runtime_http::IdentitySpec::new(1u8, 2u8, 3u8, 4u8),
            ])
            .expect("743c519b plan_preserves_typed_identity_specs invariant must hold"),
        );
        let identity = plan
            .identities()
            .first()
            .expect("b9368d0c plan_preserves_typed_identity_specs invariant must hold");
        assert_eq!(identity.login(), &1u8);
    }

    #[test]
    fn identity_specs_rejects_more_than_supported_entries() {
        let values =
            std::iter::repeat_with(|| server_runtime_http::IdentitySpec::new(1u8, 2u8, 3u8, 4u8))
                .take(
                    super::DEVELOPMENT_IDENTITY_SPECS_MAX_LEN.saturating_add(constants_usize::ONE),
                )
                .collect::<Vec<_>>();
        assert_eq!(
            super::DevelopmentIdentitySpecs::try_from(values),
            Err(super::DevelopmentIdentitySpecsError)
        );
    }
}
