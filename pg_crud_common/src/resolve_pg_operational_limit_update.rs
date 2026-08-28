pub fn resolve_pg_operational_limit_update(
    current: crate::domain_types::PgOperationalLimit,
    requested: crate::domain_types::PgOperationalLimit,
    current_usage: crate::domain_types::PgCounterValue,
    authority: crate::domain_types::PgOperationalLimitUpdateAuthority,
) -> Result<crate::domain_types::PgOperationalLimit, crate::domain_types::PgOperationalLimitError> {
    match authority {
        crate::domain_types::PgOperationalLimitUpdateAuthority::MigrationDefault => {
            Ok(current.max(requested))
        }
        crate::domain_types::PgOperationalLimitUpdateAuthority::Operator
            if requested.0.get() < current_usage.0 =>
        {
            Err(crate::domain_types::PgOperationalLimitError::BelowCurrentUsage)
        }
        crate::domain_types::PgOperationalLimitUpdateAuthority::Operator => Ok(requested),
    }
}

#[cfg(test)]
mod tests {
    fn limit(value: u64) -> crate::domain_types::PgOperationalLimit {
        crate::domain_types::PgOperationalLimit::try_from(value)
            .expect("2710e8b4 limit invariant must hold")
    }

    #[test]
    fn migration_defaults_only_raise_limits_and_operator_cannot_cross_usage() {
        assert_eq!(
            super::resolve_pg_operational_limit_update(
                limit(100u64),
                limit(50u64),
                80u64.into(),
                crate::domain_types::PgOperationalLimitUpdateAuthority::MigrationDefault,
            ),
            Ok(limit(100u64))
        );
        assert_eq!(
            super::resolve_pg_operational_limit_update(
                limit(100u64),
                limit(50u64),
                80u64.into(),
                crate::domain_types::PgOperationalLimitUpdateAuthority::Operator,
            ),
            Err(crate::domain_types::PgOperationalLimitError::BelowCurrentUsage)
        );
    }
}
