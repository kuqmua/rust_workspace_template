pub fn resolve_pg_operational_limit_update(
    current: crate::pg_operational_limit::PgOperationalLimit,
    requested: crate::pg_operational_limit::PgOperationalLimit,
    current_usage: crate::pg_counter_value::PgCounterValue,
    authority: crate::pg_operational_limit_update_authority::PgOperationalLimitUpdateAuthority,
) -> Result<
    crate::pg_operational_limit::PgOperationalLimit,
    crate::pg_operational_limit_error::PgOperationalLimitError,
> {
    match authority {
        crate::pg_operational_limit_update_authority::PgOperationalLimitUpdateAuthority::MigrationDefault => {
            Ok(current.max(requested))
        }
        crate::pg_operational_limit_update_authority::PgOperationalLimitUpdateAuthority::Operator
            if requested.get_inner().get() < *current_usage.get_inner() =>
        {
            Err(crate::pg_operational_limit_error::PgOperationalLimitError::BelowCurrentUsage)
        }
        crate::pg_operational_limit_update_authority::PgOperationalLimitUpdateAuthority::Operator => Ok(requested),
    }
}

#[cfg(test)]
mod tests {
    fn limit(value: u64) -> crate::pg_operational_limit::PgOperationalLimit {
        crate::pg_operational_limit::PgOperationalLimit::try_from(value)
            .expect("2710e8b4 limit invariant must hold")
    }

    #[test]
    fn test_migration_defaults_only_raise_limits_and_operator_cannot_cross_usage() {
        assert_eq!(
            crate::resolve_pg_operational_limit_update::resolve_pg_operational_limit_update(
                limit(100u64),
                limit(50u64),
                80u64.into(),
                crate::pg_operational_limit_update_authority::PgOperationalLimitUpdateAuthority::MigrationDefault,
            ),
            Ok(limit(100u64))
        );
        assert_eq!(
            crate::resolve_pg_operational_limit_update::resolve_pg_operational_limit_update(
                limit(100u64),
                limit(50u64),
                80u64.into(),
                crate::pg_operational_limit_update_authority::PgOperationalLimitUpdateAuthority::Operator,
            ),
            Err(crate::pg_operational_limit_error::PgOperationalLimitError::BelowCurrentUsage)
        );
    }
}
