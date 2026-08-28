use super::StdBool;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
pub(crate) struct ContractStructApiArgs {
    pub(crate) into_parts: StdBool,
    pub(crate) new: StdBool,
}
