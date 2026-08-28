#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct ProcessCommand {
    pub(super) arguments: crate::domain_types::ProcessArguments,
    pub(super) program: crate::domain_types::ProcessProgram,
}

impl ProcessCommand {
    #[must_use]
    pub const fn arguments(&self) -> &crate::domain_types::ProcessArguments {
        &self.arguments
    }

    #[must_use]
    pub const fn program(&self) -> crate::domain_types::ProcessProgram {
        self.program
    }
}
