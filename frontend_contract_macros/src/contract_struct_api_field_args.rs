use super::{StdBool, SynType};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "each flag independently opts one field into a distinct generated method"
)]
pub(crate) struct ContractStructApiFieldArgs {
    pub(crate) slice: Option<SynType>,
    pub(crate) borrow: StdBool,
    pub(crate) copy: StdBool,
    pub(crate) copy_ref: StdBool,
    pub(crate) into: StdBool,
    pub(crate) option_borrow: StdBool,
}
