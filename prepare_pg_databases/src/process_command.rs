#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct ProcessCommand {
    arguments: crate::process_arguments::ProcessArguments,
    program: crate::process_program::ProcessProgram,
}
impl
    From<(
        crate::process_program::ProcessProgram,
        crate::process_arguments::ProcessArguments,
    )> for ProcessCommand
{
    fn from(
        (program, arguments): (
            crate::process_program::ProcessProgram,
            crate::process_arguments::ProcessArguments,
        ),
    ) -> Self {
        Self { arguments, program }
    }
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
