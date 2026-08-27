#[path = "migrations/admin_migrator.rs"]
mod admin_migrator;
#[path = "migrations/create_initial_administrator.rs"]
mod create_initial_administrator;
#[path = "migrations/migrator.rs"]
mod migrator;
#[path = "migrations/reset_admin_password.rs"]
mod reset_admin_password;
#[path = "migrations/sqlx_admin_migrator_ref.rs"]
mod sqlx_admin_migrator_ref;

use admin_migrator::ADMIN_MIGRATOR;
pub(crate) use create_initial_administrator::create_initial_administrator;
pub(crate) use migrator::migrator;
pub(crate) use reset_admin_password::reset_admin_password;
pub(crate) use sqlx_admin_migrator_ref::SqlxAdminMigratorRef;
