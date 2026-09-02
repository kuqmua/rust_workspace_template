#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    serde::Deserialize,
)]
#[serde(from = "bool")]
#[derive(proc_macro_newtype::FromInner, proc_macro_newtype::IntoInnerFrom)]
pub(super) struct GenerateSecretText(bool);
