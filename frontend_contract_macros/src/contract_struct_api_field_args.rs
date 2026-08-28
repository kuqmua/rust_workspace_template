use crate::{StdBool, SynType};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "each flag independently opts one field into a distinct generated method"
)]
#[derive(generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct ContractStructApiFieldArgs {
    slice: Option<SynType>,
    borrow: StdBool,
    copy: StdBool,
    copy_ref: StdBool,
    into: StdBool,
    option_borrow: StdBool,
}
