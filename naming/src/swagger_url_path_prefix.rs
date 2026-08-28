#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct SwaggerUrlPathPrefix<'prefix_lt>(&'prefix_lt str);
