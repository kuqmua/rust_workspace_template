#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct ProcessCommand {
    arguments: crate::process_arguments::ProcessArguments,
    #[getters(copy)]
    program: crate::process_program::ProcessProgram,
}
impl
    From<(
        crate::process_program::ProcessProgram,
        crate::process_arguments::ProcessArguments,
    )> for ProcessCommand
{
    fn from(
        value: (
            crate::process_program::ProcessProgram,
            crate::process_arguments::ProcessArguments,
        ),
    ) -> Self {
        let (program, arguments) = value;
        Self { arguments, program }
    }
}
