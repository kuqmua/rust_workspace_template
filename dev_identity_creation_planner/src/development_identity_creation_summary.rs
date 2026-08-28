#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Default, Eq, PartialEq,
)]
pub struct DevelopmentIdentityCreationSummary {
    already_exists: super::DevelopmentIdentityCount,
    create: super::DevelopmentIdentityCount,
    missing_role: super::DevelopmentIdentityCount,
}

impl DevelopmentIdentityCreationSummary {
    #[must_use]
    pub const fn already_exists(self) -> super::DevelopmentIdentityCount {
        self.already_exists
    }

    #[must_use]
    pub const fn create(self) -> super::DevelopmentIdentityCount {
        self.create
    }

    #[must_use]
    pub const fn missing_role(self) -> super::DevelopmentIdentityCount {
        self.missing_role
    }

    pub(crate) const fn record(
        &mut self,
        decision: server_runtime_http::domain_types::IdentityCreationDecision,
    ) {
        match decision {
            server_runtime_http::domain_types::IdentityCreationDecision::AlreadyExists => {
                self.already_exists.0 = self.already_exists.0.saturating_add(constants_usize::ONE);
            }
            server_runtime_http::domain_types::IdentityCreationDecision::Create => {
                self.create.0 = self.create.0.saturating_add(constants_usize::ONE);
            }
            server_runtime_http::domain_types::IdentityCreationDecision::MissingRole => {
                self.missing_role.0 = self.missing_role.0.saturating_add(constants_usize::ONE);
            }
        }
    }
}
