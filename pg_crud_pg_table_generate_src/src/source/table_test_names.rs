#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(super) struct TableTestNames<'value_lt>(pub(super) Vec<&'value_lt str>);
