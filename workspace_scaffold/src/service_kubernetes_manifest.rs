#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
pub(super) struct ServiceKubernetesManifest(String);
