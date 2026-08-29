#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct ProcessCommand {
    pub(super) arguments: crate::process_arguments::ProcessArguments,
    pub(super) program: crate::process_program::ProcessProgram,
}

impl ProcessCommand {
    #[must_use]
    pub const fn arguments(&self) -> &crate::process_arguments::ProcessArguments {
        &self.arguments
    }

    #[must_use]
    pub const fn program(&self) -> crate::process_program::ProcessProgram {
        self.program
    }
}
