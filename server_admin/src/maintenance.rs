#[cfg(test)]
mod tests {
    #[test]
    fn test_cleanup_batch_rejects_zero() {
        assert_eq!(
            crate::admin_cleanup_batch_size::AdminCleanupBatchSize::try_from(constants_i64::ZERO),
            Err(crate::admin_cleanup_cfg_error::AdminCleanupCfgError::BatchSizeOutOfRange),
        );
    }
}

// Root-owned module compatibility wrappers.
mod admin_audit_action {}
mod admin_audit_resource {}
mod sqlx_admin_migrate_error {}
mod admin_migrate_error_inner {}
mod admin_migrate_error {}
mod prepare_postgresql {}
mod admin_cleanup_batch_size {}
mod admin_cleanup_retention_seconds {}
mod admin_cleanup_cfg {}
mod admin_cleanup_report {}
mod admin_cleanup_rows {}
mod admin_cleanup_cfg_error {}
mod admin_cleanup_error {}
mod cleanup_admin_tables {}
mod initial_administrator_creation_error {}
mod admin_password_reset_error {}
mod create_initial_administrator {}
mod reset_admin_password {}
