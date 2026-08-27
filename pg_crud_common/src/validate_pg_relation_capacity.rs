pub fn validate_pg_relation_capacity(
    current: crate::domain_types::PgRelationRowCount,
    incoming: crate::domain_types::PgRelationRowCount,
    maximum: crate::domain_types::PgRelationCapacityMaximum,
) -> Result<crate::domain_types::PgRelationRowCount, crate::domain_types::PgRelationCapacityError> {
    let projected = current
        .0
        .checked_add(incoming.0)
        .ok_or(crate::domain_types::PgRelationCapacityError::Overflow)?;
    if projected > maximum.0.0.get() {
        Err(crate::domain_types::PgRelationCapacityError::Exceeded)
    } else {
        Ok(crate::domain_types::PgRelationRowCount::from(projected))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn capacity_rejects_excess_and_overflow() {
        let maximum = crate::domain_types::PgRelationCapacityMaximum::try_from(5u64)
            .expect("4ddf36da capacity_rejects_excess_and_overflow invariant must hold");
        assert_eq!(
            super::validate_pg_relation_capacity(3u64.into(), 2u64.into(), maximum),
            Ok(crate::domain_types::PgRelationRowCount::from(5u64))
        );
        assert_eq!(
            super::validate_pg_relation_capacity(4u64.into(), 2u64.into(), maximum),
            Err(crate::domain_types::PgRelationCapacityError::Exceeded)
        );
        assert_eq!(
            super::validate_pg_relation_capacity(u64::MAX.into(), 1u64.into(), maximum),
            Err(crate::domain_types::PgRelationCapacityError::Overflow)
        );
    }
}
