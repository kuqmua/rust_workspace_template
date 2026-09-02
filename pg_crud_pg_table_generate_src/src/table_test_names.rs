#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoIterator,
)]
pub(super) struct TableTestNames<'value_lt>(Vec<&'value_lt str>);
