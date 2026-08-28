use admin_migrator::ADMIN_MIGRATOR;
pub(crate) use migrate_create_initial_administrator::migrate_create_initial_administrator;
pub(crate) use migrate_reset_admin_password::migrate_reset_admin_password;
pub(crate) use migrator::migrator;
pub(crate) use sqlx_admin_migrator_ref::SqlxAdminMigratorRef;

// Root-owned module compatibility wrappers.
mod admin_migrator {
    pub use crate::admin_migrator::*;
}
mod migrate_create_initial_administrator {
    pub use crate::migrate_create_initial_administrator::*;
}
mod migrate_reset_admin_password {
    pub use crate::migrate_reset_admin_password::*;
}
mod migrator {
    pub use crate::migrator::*;
}
mod sqlx_admin_migrator_ref {
    pub use crate::sqlx_admin_migrator_ref::*;
}
