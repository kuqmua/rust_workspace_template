#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::FromInner,
)]
pub struct RuntimeTestReport(
    bounded_types::domain_types::vector::BoundedVec<
        crate::domain_types::RuntimeTestKind,
        { constants_usize::ZERO },
        5usize,
    >,
);

impl RuntimeTestReport {
    #[must_use]
    pub const fn passed(&self) -> &[crate::domain_types::RuntimeTestKind] {
        self.0.as_slice()
    }
}
