#[allow(
    clippy::single_call_fn,
    reason = "migrator remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) fn migrator() -> crate::sqlx_admin_migrator_ref::SqlxAdminMigratorRef {
    crate::sqlx_admin_migrator_ref::SqlxAdminMigratorRef::from(
        &crate::admin_migrator::ADMIN_MIGRATOR,
    )
}
