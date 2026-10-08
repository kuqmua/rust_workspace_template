#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_from_inner::FromInner,
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_runtime_report_preserves_empty_to_full_order_and_capacity_error() {
        let kinds = [
            crate::runtime_test_kind::RuntimeTestKind::ApplicationLiveness,
            crate::runtime_test_kind::RuntimeTestKind::ApplicationReadiness,
            crate::runtime_test_kind::RuntimeTestKind::NotificationServiceLiveness,
            crate::runtime_test_kind::RuntimeTestKind::NotificationServiceReadiness,
            crate::runtime_test_kind::RuntimeTestKind::NotificationCreation,
        ];
        assert!((0usize..=5usize).all(|count| {
            let values = kinds.into_iter().take(count).collect::<Vec<_>>();
            let pointer = values.as_ptr();
            bounded_types::bounded_vec::BoundedVec::<
                crate::runtime_test_kind::RuntimeTestKind,
                0usize,
                5usize,
            >::try_from(values)
            .is_ok_and(|bounded_vec| {
                let report = crate::runtime_test_report::RuntimeTestReport::from(bounded_vec);
                report.passed().len() == count
                    && report.passed().as_ptr() == pointer
                    && report
                        .passed()
                        .iter()
                        .copied()
                        .eq(kinds.into_iter().take(count))
            })
        }));
        let oversized = kinds.into_iter().cycle().take(6usize).collect::<Vec<_>>();
        assert_eq!(
            bounded_types::bounded_vec::BoundedVec::<
                crate::runtime_test_kind::RuntimeTestKind,
                0usize,
                5usize,
            >::try_from(oversized),
            Err(
                bounded_types::bounded_value_error::BoundedValueError::AboveMax {
                    actual: bounded_types::bounded_len::BoundedLen::from(6usize),
                    max: bounded_types::bounded_len::BoundedLen::from(5usize),
                }
            ),
        );
    }
}
