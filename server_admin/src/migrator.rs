#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) fn migrator() -> crate::sqlx_admin_migrator_ref::SqlxAdminMigratorRef {
    crate::sqlx_admin_migrator_ref::SqlxAdminMigratorRef::from(
        &crate::admin_migrator::ADMIN_MIGRATOR,
    )
}
