#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    proc_macro_newtype_display::Display,
)]
pub struct StdStaleStagingEntryCount(usize);

impl StdStaleStagingEntryCount {
    pub(super) const fn increment(&mut self) {
        self.0 = self.0.saturating_add(constants_usize::ONE);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_stale_staging_count_saturates_without_wrapping() {
        let maximum = std::num::NonZeroUsize::MAX.get();
        [maximum.saturating_sub(1usize), maximum]
            .into_iter()
            .fold((), |(), value| {
                let mut std_stale_staging_entry_count =
                    super::StdStaleStagingEntryCount::from(value);
                std_stale_staging_entry_count.increment();
                assert_eq!(usize::from(std_stale_staging_entry_count), maximum);
                std_stale_staging_entry_count.increment();
                assert_eq!(usize::from(std_stale_staging_entry_count), maximum);
            });
    }
}
