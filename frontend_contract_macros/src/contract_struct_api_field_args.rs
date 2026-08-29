#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "each flag independently opts one field into a distinct generated method"
)]
#[derive(generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct ContractStructApiFieldArgs {
    slice: Option<crate::syn_type::SynType>,
    borrow: crate::std_bool::StdBool,
    copy: crate::std_bool::StdBool,
    copy_ref: crate::std_bool::StdBool,
    into: crate::std_bool::StdBool,
    option_borrow: crate::std_bool::StdBool,
}
