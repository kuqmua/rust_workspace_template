#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct ParseEnvVarNameRef<'name_lt>(&'name_lt str);
