#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Default, Eq, PartialEq,
)]
pub struct DevelopmentIdentityCreationSummary {
    #[getters(copy)]
    already_exists: crate::development_identity_count::DevelopmentIdentityCount,
    #[getters(copy)]
    create: crate::development_identity_count::DevelopmentIdentityCount,
    #[getters(copy)]
    missing_role: crate::development_identity_count::DevelopmentIdentityCount,
}

impl DevelopmentIdentityCreationSummary {
    pub(crate) const fn record(
        &mut self,
        decision: server_runtime_core::identity_creation_decision::IdentityCreationDecision,
    ) {
        match decision {
            server_runtime_core::identity_creation_decision::IdentityCreationDecision::AlreadyExists => {
                self.already_exists.increment();
            }
            server_runtime_core::identity_creation_decision::IdentityCreationDecision::Create => {
                self.create.increment();
            }
            server_runtime_core::identity_creation_decision::IdentityCreationDecision::MissingRole => {
                self.missing_role.increment();
            }
        }
    }
}
