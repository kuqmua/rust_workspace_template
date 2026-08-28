pub(crate) use admin_audit_resource_id::AdminAuditResourceId;
pub(crate) use admin_audit_success_ref::AdminAuditSuccessRef;
pub(crate) use admin_db_ref::AdminDbRef;
pub(crate) use load_authenticated_admin::load_authenticated_admin;
pub(crate) use load_authenticated_admin_from_db::load_authenticated_admin_from_db;
pub(crate) use record_audit_success_in_connection::record_audit_success_in_connection;
pub(crate) use record_login_attempt::record_login_attempt;

// Root-owned module compatibility wrappers.
mod admin_audit_resource_id {
    pub use crate::admin_audit_resource_id::*;
}
mod admin_audit_success_ref {
    pub use crate::admin_audit_success_ref::*;
}
mod admin_db_ref {
    pub use crate::admin_db_ref::*;
}
mod load_authenticated_admin {
    pub use crate::load_authenticated_admin::*;
}
mod load_authenticated_admin_from_db {
    pub use crate::load_authenticated_admin_from_db::*;
}
mod record_audit_success_in_connection {
    pub use crate::record_audit_success_in_connection::*;
}
mod record_login_attempt {
    pub use crate::record_login_attempt::*;
}
