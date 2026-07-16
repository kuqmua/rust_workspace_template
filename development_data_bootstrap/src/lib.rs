#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DevelopmentBootstrapPlan<Login, DisplayName, Role, SecretSource> {
    identities: Vec<server_runtime::IdentitySpec<Login, DisplayName, Role, SecretSource>>,
}

impl<Login, DisplayName, Role, SecretSource>
    DevelopmentBootstrapPlan<Login, DisplayName, Role, SecretSource>
{
    #[must_use]
    pub fn identities(
        &self,
    ) -> &[server_runtime::IdentitySpec<Login, DisplayName, Role, SecretSource>] {
        &self.identities
    }

    #[must_use]
    pub const fn new(
        identities: Vec<server_runtime::IdentitySpec<Login, DisplayName, Role, SecretSource>>,
    ) -> Self {
        Self { identities }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DevelopmentBootstrapSummary {
    already_exists: DevelopmentIdentityCount,
    create: DevelopmentIdentityCount,
    missing_role: DevelopmentIdentityCount,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::IntoInnerFrom)]
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
}

#[must_use]
pub fn summarize_identity_bootstrap<Reports>(reports: Reports) -> DevelopmentBootstrapSummary
where
    Reports: IntoIterator<Item = server_runtime::IdentityBootstrapReport>,
{
    reports.into_iter().fold(
        DevelopmentBootstrapSummary {
            already_exists: DevelopmentIdentityCount(0usize),
            create: DevelopmentIdentityCount(0usize),
            missing_role: DevelopmentIdentityCount(0usize),
        },
        |mut summary, report| {
            match report.decision() {
                server_runtime::IdentityBootstrapDecision::AlreadyExists => {
                    summary.already_exists.0 = summary.already_exists.0.saturating_add(1usize);
                }
                server_runtime::IdentityBootstrapDecision::Create => {
                    summary.create.0 = summary.create.0.saturating_add(1usize);
                }
                server_runtime::IdentityBootstrapDecision::MissingRole => {
                    summary.missing_role.0 = summary.missing_role.0.saturating_add(1usize);
                }
            }
            summary
        },
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn summarizes_desired_state_decisions() {
        let reports = [
            server_runtime::plan_identity_bootstrap(
                server_runtime::IdentityPresence::Missing,
                server_runtime::IdentityRolePresence::Present,
            ),
            server_runtime::plan_identity_bootstrap(
                server_runtime::IdentityPresence::Present,
                server_runtime::IdentityRolePresence::Present,
            ),
            server_runtime::plan_identity_bootstrap(
                server_runtime::IdentityPresence::Missing,
                server_runtime::IdentityRolePresence::Missing,
            ),
        ];
        let summary = super::summarize_identity_bootstrap(reports);
        assert_eq!(usize::from(summary.create()), 1usize);
        assert_eq!(usize::from(summary.already_exists()), 1usize);
        assert_eq!(usize::from(summary.missing_role()), 1usize);
    }

    #[test]
    fn plan_preserves_typed_identity_specs() {
        let plan = super::DevelopmentBootstrapPlan::new(vec![server_runtime::IdentitySpec::new(
            1u8, 2u8, 3u8, 4u8,
        )]);
        let identity = plan.identities().first().expect("b9368d0c");
        assert_eq!(identity.login(), &1u8);
    }
}
