#![allow(clippy::single_call_fn)] // separate same-named owner module preserves the migrations boundary
pub(crate) fn migrator() -> super::SqlxAdminMigratorRef {
    super::SqlxAdminMigratorRef::from(&super::ADMIN_MIGRATOR)
}
