#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefOwned,
    newtype::FromInner,
)]
pub struct BatchRecordsBTreeMap<Key, Record>(std::collections::BTreeMap<Key, Record>);
