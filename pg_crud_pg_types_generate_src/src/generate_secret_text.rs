#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    serde::Deserialize,
)]
#[serde(from = "bool")]
#[derive(
    proc_macro_newtype_from_inner::FromInner, proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub(super) struct GenerateSecretText(bool);
