#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub struct JsonFixtureRef<'fixture_lt>(&'fixture_lt str);

impl<'fixture_lt> JsonFixtureRef<'fixture_lt> {
    pub(crate) const fn as_str(self) -> &'fixture_lt str {
        self.0
    }
}
