#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Default, serde::Deserialize,
)]
#[serde(from = "bool")]
#[derive(newtype::FromInner, newtype::IntoInnerFrom)]
pub(super) struct GenerateSecretText(bool);
