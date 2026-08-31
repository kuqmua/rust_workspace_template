#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoIterator)]
pub(super) struct TableTestNames<'value_lt>(Vec<&'value_lt str>);
