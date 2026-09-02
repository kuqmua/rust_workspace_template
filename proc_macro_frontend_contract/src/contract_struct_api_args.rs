#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default, proc_macro_getters::Getters,
)]
#[getters(get_mut)]
pub(crate) struct ContractStructApiArgs {
    into_parts: crate::std_bool::StdBool,
    generate_constructor: crate::std_bool::StdBool,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl ContractStructApiArgs {}
