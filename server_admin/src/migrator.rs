#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) fn migrator() -> crate::SqlxAdminMigratorRef {
    crate::SqlxAdminMigratorRef::from(&crate::ADMIN_MIGRATOR)
}
