#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct DevelopmentIdentityCount(usize);

impl DevelopmentIdentityCount {
    pub(super) const fn increment(&mut self) {
        self.0 = self.0.saturating_add(constants_usize::ONE);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_development_identity_count_saturates_at_native_maximum() {
        let maximum = std::num::NonZeroUsize::MAX.get();
        [maximum.saturating_sub(constants_usize::ONE), maximum]
            .into_iter()
            .fold((), |(), value| {
                let mut development_identity_count = super::DevelopmentIdentityCount::from(value);
                development_identity_count.increment();
                assert_eq!(usize::from(development_identity_count), maximum);
                development_identity_count.increment();
                assert_eq!(usize::from(development_identity_count), maximum);
            });
    }
}
