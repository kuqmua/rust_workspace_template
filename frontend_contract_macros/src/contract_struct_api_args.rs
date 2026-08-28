use crate::StdBool;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct ContractStructApiArgs {
    into_parts: StdBool,
    new: StdBool,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl ContractStructApiArgs {}
