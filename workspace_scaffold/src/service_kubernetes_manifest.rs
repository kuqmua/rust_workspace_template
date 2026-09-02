#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::BoundedStringWrapper,
)]
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
pub(super) struct ServiceKubernetesManifest(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { constants_usize::VALUE_16_777_216 },
        false,
    >,
);
