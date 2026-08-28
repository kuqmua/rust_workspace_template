#![allow(clippy::field_scoped_visibility_modifiers)] // sibling domain modules require raw representations while facade reexports must keep fields externally private

mod admin_audit_log_id;
mod admin_id_try_from_i64_error;
mod admin_non_zero_usize;
mod admin_permission_id;
mod admin_permission_name;
mod admin_resource_text;
mod admin_role_id;
mod admin_socket_addr;
mod admin_user_id;
pub mod domain_types;
mod secrecy_admin_string;
mod std_admin_bool;
mod std_admin_str_ref;
mod std_admin_string;
mod uuid_admin_value;
