use admin_migrator::ADMIN_MIGRATOR;
pub(crate) use migrator::migrator;
pub(crate) use sqlx_admin_migrator_ref::SqlxAdminMigratorRef;

// Root-owned module compatibility wrappers.
mod admin_migrator {
    pub use crate::admin_migrator::*;
}
mod migrator {
    pub use crate::migrator::*;
}
mod sqlx_admin_migrator_ref {
    pub use crate::sqlx_admin_migrator_ref::*;
}
