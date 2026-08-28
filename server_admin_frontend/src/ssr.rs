#![allow(
    clippy::unused_trait_names,
    reason = "the stable SSR facade delegates to screen, document, and table modules; test view rendering requires the named extension trait"
)]

pub use crate::admin_ssr_error_message::*;
pub use crate::admin_ssr_html::*;
pub use crate::admin_ssr_html_try_from_string_error::*;
pub use crate::admin_ssr_text::*;
pub use crate::admin_ssr_text_try_from_string_error::*;
#[cfg(test)]
use crate::admin_ssr_view_ext::*;
pub use crate::render_admin_permissions_page::*;
pub use crate::render_admin_profile_page::*;
pub use crate::render_admin_sessions_page::*;
pub use crate::render_admin_settings_page::*;
pub use crate::render_role_create::*;
pub use crate::render_role_manage::*;
pub use crate::render_user_create::*;
pub use crate::render_user_manage::*;
use crate::render_view::render_view;

pub use self::data_tables::render_admin_csr::render_admin_csr;
pub use self::data_tables::render_data_tables::render_data_tables;
pub use self::data_tables::render_data_tables_csr::render_data_tables_csr;
pub use self::document::render_sign_in::render_sign_in;
pub use self::render_roles::render_roles;
pub use self::render_users::render_users;
pub use self::text_page::render_text_page::render_text_page;
pub use self::text_page::render_text_page_with_access::render_text_page_with_access;

use self::document::render_admin_page::render_admin_page;
use self::document::render_admin_page_with_access::render_admin_page_with_access;
use self::document::render_admin_page_with_table_access::render_admin_page_with_table_access;
use self::document::render_document::render_document;
use crate::data_table_grid::data_table_grid;
use crate::table_pagination::table_pagination;

// Root-owned module compatibility wrappers.
pub(crate) mod admin_ssr_html_try_from_string_error {
    pub use crate::admin_ssr_html_try_from_string_error::*;
}
pub(crate) mod crud_render_role_create {
    pub use crate::crud_render_role_create::*;
}
pub(crate) mod crud_render_role_manage {
    pub use crate::crud_render_role_manage::*;
}
pub(crate) mod crud_render_shell {
    pub use crate::crud_render_shell::*;
}
pub(crate) mod crud_render_user_create {
    pub use crate::crud_render_user_create::*;
}
pub(crate) mod crud_render_user_manage {
    pub use crate::crud_render_user_manage::*;
}
pub(crate) mod data_table_grid {
    pub use crate::data_table_grid::*;
}
pub(crate) mod data_tables {
    pub use crate::data_tables::*;
}
pub(crate) mod document {
    pub use crate::domain_types_ssr_document::*;
}
pub(crate) mod render_permissions {
    pub use crate::render_permissions::*;
}
pub(crate) mod render_profile {
    pub use crate::render_profile::*;
}
pub(crate) mod render_roles {
    pub use crate::render_roles::*;
}
pub(crate) mod render_sessions {
    pub use crate::render_sessions::*;
}
pub(crate) mod render_settings {
    pub use crate::render_settings::*;
}
pub(crate) mod render_users {
    pub use crate::render_users::*;
}
pub(crate) mod table_pagination {
    pub use crate::table_pagination::*;
}
pub(crate) mod text_page {
    pub use crate::text_page::*;
}
pub(crate) mod admin_ssr_text_try_from_string_error {
    pub use crate::admin_ssr_text_try_from_string_error::*;
}
pub(crate) mod admin_ssr_error_message {
    pub use crate::admin_ssr_error_message::*;
}
pub(crate) mod admin_ssr_text {
    pub use crate::admin_ssr_text::*;
}
pub(crate) mod admin_ssr_html {
    pub use crate::admin_ssr_html::*;
}
pub(crate) mod render_user_create {
    pub use crate::render_user_create::*;
}
pub(crate) mod render_user_manage {
    pub use crate::render_user_manage::*;
}
pub(crate) mod render_role_create {
    pub use crate::render_role_create::*;
}
pub(crate) mod render_role_manage {
    pub use crate::render_role_manage::*;
}
pub(crate) mod render_admin_permissions_page {
    pub use crate::render_admin_permissions_page::*;
}
pub(crate) mod render_admin_sessions_page {
    pub use crate::render_admin_sessions_page::*;
}
pub(crate) mod render_admin_profile_page {
    pub use crate::render_admin_profile_page::*;
}
pub(crate) mod render_admin_settings_page {
    pub use crate::render_admin_settings_page::*;
}
pub(crate) mod render_view {
    pub use crate::render_view::*;
}
