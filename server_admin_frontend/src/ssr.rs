pub use self::data_tables::render_admin_csr::render_admin_csr;
pub use self::data_tables::render_data_tables::render_data_tables;
pub use self::data_tables::render_data_tables_csr::render_data_tables_csr;
pub use self::document::render_sign_in::render_sign_in;
pub use self::render_roles::render_roles;
pub use self::render_users::render_users;
pub use self::text_page::render_text_page::render_text_page;
pub use self::text_page::render_text_page_with_access::render_text_page_with_access;
pub use super::admin_ssr_error_message::*;
pub use super::admin_ssr_html::*;
pub use super::admin_ssr_html_try_from_string_error::*;
pub use super::admin_ssr_text::*;
pub use super::admin_ssr_text_try_from_string_error::*;
#[cfg(test)]
use super::admin_ssr_view_ext::*;
pub use super::crud_render_role_create::*;
pub use super::crud_render_role_manage::*;
pub use super::crud_render_user_create::*;
pub use super::crud_render_user_manage::*;
pub use super::render_permissions::*;
pub use super::render_profile::*;
pub use super::render_sessions::*;
pub use super::render_settings::*;
use super::render_view::render_view;

use self::document::render_admin_page::render_admin_page;
use self::document::render_admin_page_with_access::render_admin_page_with_access;
use self::document::render_admin_page_with_table_access::render_admin_page_with_table_access;
use self::document::render_document::render_document;
use super::data_table_grid::data_table_grid;
use super::table_pagination::table_pagination;

// Root-owned module compatibility wrappers.
pub(crate) mod admin_ssr_html_try_from_string_error {
    pub use super::super::admin_ssr_html_try_from_string_error::*;
}
pub(crate) mod crud_render_role_create {
    pub use super::super::crud_render_role_create::*;
}
pub(crate) mod crud_render_role_manage {
    pub use super::super::crud_render_role_manage::*;
}
pub(crate) mod crud_render_shell {
    pub use super::super::crud_render_shell::*;
}
pub(crate) mod crud_render_user_create {
    pub use super::super::crud_render_user_create::*;
}
pub(crate) mod crud_render_user_manage {
    pub use super::super::crud_render_user_manage::*;
}
pub(crate) mod data_table_grid {
    pub use super::super::data_table_grid::*;
}
pub(crate) mod data_tables {
    pub use super::super::data_tables::*;
}
pub(crate) mod document {
    pub use super::super::domain_types_ssr_document::*;
}
pub(crate) mod render_permissions {
    pub use super::super::render_permissions::*;
}
pub(crate) mod render_profile {
    pub use super::super::render_profile::*;
}
pub(crate) mod render_roles {
    pub use super::super::render_roles::*;
}
pub(crate) mod render_sessions {
    pub use super::super::render_sessions::*;
}
pub(crate) mod render_settings {
    pub use super::super::render_settings::*;
}
pub(crate) mod render_users {
    pub use super::super::render_users::*;
}
pub(crate) mod table_pagination {
    pub use super::super::table_pagination::*;
}
pub(crate) mod text_page {
    pub use super::super::text_page::*;
}
pub(crate) mod admin_ssr_text_try_from_string_error {
    pub use super::super::admin_ssr_text_try_from_string_error::*;
}
pub(crate) mod admin_ssr_error_message {
    pub use super::super::admin_ssr_error_message::*;
}
pub(crate) mod admin_ssr_text {
    pub use super::super::admin_ssr_text::*;
}
pub(crate) mod admin_ssr_html {
    pub use super::super::admin_ssr_html::*;
}
pub(crate) mod render_user_create {
    pub use super::super::crud_render_user_create::*;
}
pub(crate) mod render_user_manage {
    pub use super::super::crud_render_user_manage::*;
}
pub(crate) mod render_role_create {
    pub use super::super::crud_render_role_create::*;
}
pub(crate) mod render_role_manage {
    pub use super::super::crud_render_role_manage::*;
}
pub(crate) mod render_admin_permissions_page {
    pub use super::super::render_permissions::*;
}
pub(crate) mod render_admin_sessions_page {
    pub use super::super::render_sessions::*;
}
pub(crate) mod render_admin_profile_page {
    pub use super::super::render_profile::*;
}
pub(crate) mod render_admin_settings_page {
    pub use super::super::render_settings::*;
}
pub(crate) mod render_view {
    pub use super::super::render_view::*;
}
