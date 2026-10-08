pub fn resolve_pg_operational_limit_update(
    current: crate::pg_operational_limit::PgOperationalLimit,
    requested: crate::pg_operational_limit::PgOperationalLimit,
    pg_counter_value: crate::pg_counter_value::PgCounterValue,
    pg_operational_limit_update_authority: crate::pg_operational_limit_update_authority::PgOperationalLimitUpdateAuthority,
) -> Result<
    crate::pg_operational_limit::PgOperationalLimit,
    crate::pg_operational_limit_error::PgOperationalLimitError,
> {
    match pg_operational_limit_update_authority {
        crate::pg_operational_limit_update_authority::PgOperationalLimitUpdateAuthority::MigrationDefault => {
            Ok(current.max(requested))
        }
        crate::pg_operational_limit_update_authority::PgOperationalLimitUpdateAuthority::Operator
            if requested.get_inner().get() < *pg_counter_value.get_inner() =>
        {
            Err(crate::pg_operational_limit_error::PgOperationalLimitError::BelowCurrentUsage)
        }
        crate::pg_operational_limit_update_authority::PgOperationalLimitUpdateAuthority::Operator => Ok(requested),
    }
}

#[cfg(test)]
mod tests {
    fn limit(u64: u64) -> crate::pg_operational_limit::PgOperationalLimit {
        crate::pg_operational_limit::PgOperationalLimit::try_from(u64)
            .expect(constants_str::DIAGNOSTIC_2710E8B4)
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
    #[test]
    fn test_operational_limit_updates_preserve_authority_and_usage_boundaries() {
        assert!([
            (100u64, 150u64, 80u64, 150u64),
            (100u64, 100u64, 80u64, 100u64),
            (1u64, u64::MAX, 0u64, u64::MAX),
            (u64::MAX, 1u64, u64::MAX, u64::MAX),
        ].into_iter().all(|(current, requested, usage, expected)| {
            crate::resolve_pg_operational_limit_update::resolve_pg_operational_limit_update(
                limit(current), limit(requested), usage.into(),
                crate::pg_operational_limit_update_authority::PgOperationalLimitUpdateAuthority::MigrationDefault,
            ) == Ok(limit(expected))
        }));
        assert!([
            (100u64, 50u64, 49u64, Ok(50u64)),
            (100u64, 50u64, 50u64, Ok(50u64)),
            (100u64, 50u64, 51u64, Err(crate::pg_operational_limit_error::PgOperationalLimitError::BelowCurrentUsage)),
            (u64::MAX, 1u64, 1u64, Ok(1u64)),
            (1u64, u64::MAX, u64::MAX, Ok(u64::MAX)),
            (u64::MAX, 1u64, u64::MAX, Err(crate::pg_operational_limit_error::PgOperationalLimitError::BelowCurrentUsage)),
        ].into_iter().all(|(current, requested, usage, expected)| {
            crate::resolve_pg_operational_limit_update::resolve_pg_operational_limit_update(
                limit(current), limit(requested), usage.into(),
                crate::pg_operational_limit_update_authority::PgOperationalLimitUpdateAuthority::Operator,
            ) == expected.map(limit)
        }));
    }
}
