pub fn validate_pg_relation_capacity(
    current: crate::pg_relation_row_count::PgRelationRowCount,
    incoming: crate::pg_relation_row_count::PgRelationRowCount,
    pg_relation_capacity_maximum: crate::pg_relation_capacity_maximum::PgRelationCapacityMaximum,
) -> Result<
    crate::pg_relation_row_count::PgRelationRowCount,
    crate::pg_relation_capacity_error::PgRelationCapacityError,
> {
    let projected = current
        .get_inner()
        .checked_add(*incoming.get_inner())
        .ok_or(crate::pg_relation_capacity_error::PgRelationCapacityError::Overflow)?;
    if projected > pg_relation_capacity_maximum.get_inner().get() {
        Err(crate::pg_relation_capacity_error::PgRelationCapacityError::Exceeded)
    } else {
        Ok(crate::pg_relation_row_count::PgRelationRowCount::from(
            projected,
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_capacity_rejects_excess_and_overflow() {
        let maximum =
            crate::pg_relation_capacity_maximum::PgRelationCapacityMaximum::try_from(5u64)
                .expect(constants_str::DIAGNOSTIC_4DDF36DA);
        assert_eq!(
            crate::validate_pg_relation_capacity::validate_pg_relation_capacity(
                3u64.into(),
                2u64.into(),
                maximum
            ),
            Ok(crate::pg_relation_row_count::PgRelationRowCount::from(5u64))
        );
        assert_eq!(
            crate::validate_pg_relation_capacity::validate_pg_relation_capacity(
                4u64.into(),
                2u64.into(),
                maximum
            ),
            Err(crate::pg_relation_capacity_error::PgRelationCapacityError::Exceeded)
        );
        assert_eq!(
            crate::validate_pg_relation_capacity::validate_pg_relation_capacity(
                u64::MAX.into(),
                1u64.into(),
                maximum
            ),
            Err(crate::pg_relation_capacity_error::PgRelationCapacityError::Overflow)
        );
    }
}
