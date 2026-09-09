#[derive(
    proc_macro_getters::Getters,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_from_inner::FromInner,
)]
#[serde(from = "Vec<Item>")]
pub struct ListItems<Item>(Vec<Item>);
