#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Default, Eq, PartialEq,
)]
pub struct DevelopmentIdentityCreationSummary {
    already_exists: crate::development_identity_count::DevelopmentIdentityCount,
    create: crate::development_identity_count::DevelopmentIdentityCount,
    missing_role: crate::development_identity_count::DevelopmentIdentityCount,
}

impl DevelopmentIdentityCreationSummary {
    #[must_use]
    pub const fn already_exists(
        self,
    ) -> crate::development_identity_count::DevelopmentIdentityCount {
        self.already_exists
    }

    #[must_use]
    pub const fn create(self) -> crate::development_identity_count::DevelopmentIdentityCount {
        self.create
    }

    #[must_use]
    pub const fn missing_role(self) -> crate::development_identity_count::DevelopmentIdentityCount {
        self.missing_role
    }

    pub(crate) const fn record(
        &mut self,
        decision: server_runtime_core::identity_creation_decision::IdentityCreationDecision,
    ) {
        match decision {
            server_runtime_core::identity_creation_decision::IdentityCreationDecision::AlreadyExists => {
                self.already_exists.0 = self.already_exists.0.saturating_add(constants_usize::ONE);
            }
            server_runtime_core::identity_creation_decision::IdentityCreationDecision::Create => {
                self.create.0 = self.create.0.saturating_add(constants_usize::ONE);
            }
            server_runtime_core::identity_creation_decision::IdentityCreationDecision::MissingRole => {
                self.missing_role.0 = self.missing_role.0.saturating_add(constants_usize::ONE);
            }
        }
    }
}
