#[path = "admin_migrator.rs"]
mod admin_migrator;
#[path = "migrate_create_initial_administrator.rs"]
mod migrate_create_initial_administrator;
#[path = "migrate_reset_admin_password.rs"]
mod migrate_reset_admin_password;
#[path = "migrator.rs"]
mod migrator;
#[path = "sqlx_admin_migrator_ref.rs"]
mod sqlx_admin_migrator_ref;

use admin_migrator::ADMIN_MIGRATOR;
pub(crate) use migrate_create_initial_administrator::migrate_create_initial_administrator;
pub(crate) use migrate_reset_admin_password::migrate_reset_admin_password;
pub(crate) use migrator::migrator;
pub(crate) use sqlx_admin_migrator_ref::SqlxAdminMigratorRef;
