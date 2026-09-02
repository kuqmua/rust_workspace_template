#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::FromInner,
)]
pub struct RuntimeTestReport(
    bounded_types::bounded_vec::BoundedVec<
        crate::runtime_test_kind::RuntimeTestKind,
        { constants_usize::ZERO },
        5usize,
    >,
);

impl RuntimeTestReport {
    #[must_use]
    pub const fn passed(&self) -> &[crate::runtime_test_kind::RuntimeTestKind] {
        self.0.as_slice()
    }
}
