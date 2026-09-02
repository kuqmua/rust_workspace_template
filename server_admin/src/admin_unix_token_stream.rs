#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    proc_macro_newtype::FromInner,
)]
#[serde(from = "u64")]
#[derive(proc_macro_getters::Getters)]
pub struct AdminUnixTokenStream(u64);
