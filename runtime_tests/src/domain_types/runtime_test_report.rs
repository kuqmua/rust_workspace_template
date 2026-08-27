#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::FromInner,
)]
pub struct RuntimeTestReport(
    bounded_types::domain_types::vector::BoundedVec<
        super::RuntimeTestKind,
        { constants_usize::ZERO },
        5usize,
    >,
);

impl RuntimeTestReport {
    #[must_use]
    pub const fn passed(&self) -> &[super::RuntimeTestKind] {
        self.0.as_slice()
    }
}
