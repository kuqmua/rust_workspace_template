#[must_use]
pub fn reconcile_pg_counter(
    tracked: crate::domain_types::PgCounterValue,
    actual: crate::domain_types::PgCounterValue,
) -> crate::domain_types::PgCounterReconciliation {
    match actual.0.cmp(&tracked.0) {
        std::cmp::Ordering::Greater => crate::domain_types::PgCounterReconciliation::ActualAhead(
            crate::domain_types::PgCounterValue::from(actual.0.saturating_sub(tracked.0)),
        ),
        std::cmp::Ordering::Less => crate::domain_types::PgCounterReconciliation::TrackedAhead(
            crate::domain_types::PgCounterValue::from(tracked.0.saturating_sub(actual.0)),
        ),
        std::cmp::Ordering::Equal => crate::domain_types::PgCounterReconciliation::InSync,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn counter_reconciliation_reports_direction_and_distance() {
        assert_eq!(
            super::reconcile_pg_counter(7u64.into(), 10u64.into()),
            crate::domain_types::PgCounterReconciliation::ActualAhead(3u64.into())
        );
        assert_eq!(
            super::reconcile_pg_counter(12u64.into(), 10u64.into()),
            crate::domain_types::PgCounterReconciliation::TrackedAhead(2u64.into())
        );
        assert_eq!(
            super::reconcile_pg_counter(10u64.into(), 10u64.into()),
            crate::domain_types::PgCounterReconciliation::InSync
        );
    }
}
