#[must_use]
pub fn reconcile_pg_counter(
    tracked: crate::pg_counter_value::PgCounterValue,
    actual: crate::pg_counter_value::PgCounterValue,
) -> crate::pg_counter_reconciliation::PgCounterReconciliation {
    match actual.get_inner().cmp(tracked.get_inner()) {
        std::cmp::Ordering::Greater => {
            crate::pg_counter_reconciliation::PgCounterReconciliation::ActualAhead(
                crate::pg_counter_value::PgCounterValue::from(
                    actual.get_inner().saturating_sub(*tracked.get_inner()),
                ),
            )
        }
        std::cmp::Ordering::Less => {
            crate::pg_counter_reconciliation::PgCounterReconciliation::TrackedAhead(
                crate::pg_counter_value::PgCounterValue::from(
                    tracked.get_inner().saturating_sub(*actual.get_inner()),
                ),
            )
        }
        std::cmp::Ordering::Equal => {
            crate::pg_counter_reconciliation::PgCounterReconciliation::InSync
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn counter_reconciliation_reports_direction_and_distance() {
        assert_eq!(
            crate::reconcile_pg_counter::reconcile_pg_counter(7u64.into(), 10u64.into()),
            crate::pg_counter_reconciliation::PgCounterReconciliation::ActualAhead(3u64.into())
        );
        assert_eq!(
            crate::reconcile_pg_counter::reconcile_pg_counter(12u64.into(), 10u64.into()),
            crate::pg_counter_reconciliation::PgCounterReconciliation::TrackedAhead(2u64.into())
        );
        assert_eq!(
            crate::reconcile_pg_counter::reconcile_pg_counter(10u64.into(), 10u64.into()),
            crate::pg_counter_reconciliation::PgCounterReconciliation::InSync
        );
    }
}
