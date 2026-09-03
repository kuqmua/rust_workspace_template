#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) fn migrator() -> crate::sqlx_admin_migrator_ref::SqlxAdminMigratorRef {
    crate::sqlx_admin_migrator_ref::SqlxAdminMigratorRef::from(
        &crate::admin_migrator::ADMIN_MIGRATOR,
    )
}
