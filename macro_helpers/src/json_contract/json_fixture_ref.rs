#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct JsonFixtureRef<'fixture_lt>(pub(super) &'fixture_lt str);
