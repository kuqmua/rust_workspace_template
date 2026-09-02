#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::FromInner,
)]
pub struct BatchRecordsBTreeMap<Key, Record>(std::collections::BTreeMap<Key, Record>);
