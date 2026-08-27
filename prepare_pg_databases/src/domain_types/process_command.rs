#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct ProcessCommand {
    pub(super) arguments: super::ProcessArguments,
    pub(super) program: super::ProcessProgram,
}

impl ProcessCommand {
    #[must_use]
    pub const fn arguments(&self) -> &super::ProcessArguments {
        &self.arguments
    }

    #[must_use]
    pub const fn program(&self) -> super::ProcessProgram {
        self.program
    }
}
