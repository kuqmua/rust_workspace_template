#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the stable SSR facade delegates to screen, document, and table modules; test view rendering requires the named extension trait"
)]

#[path = "ssr/admin_ssr_html_try_from_string_error.rs"]
mod admin_ssr_html_try_from_string_error;
#[path = "crud_render_role_create.rs"]
mod crud_render_role_create;
#[path = "crud_render_role_manage.rs"]
mod crud_render_role_manage;
#[path = "crud_render_shell.rs"]
mod crud_render_shell;
#[path = "crud_render_user_create.rs"]
mod crud_render_user_create;
#[path = "crud_render_user_manage.rs"]
mod crud_render_user_manage;
#[path = "data_table_grid.rs"]
mod data_table_grid;
#[path = "data_tables.rs"]
mod data_tables;
#[path = "domain_types_ssr_document.rs"]
mod document;
#[path = "render_permissions.rs"]
mod render_permissions;
#[path = "render_profile.rs"]
mod render_profile;
#[path = "render_roles.rs"]
mod render_roles;
#[path = "render_sessions.rs"]
mod render_sessions;
#[path = "render_settings.rs"]
mod render_settings;
#[path = "render_users.rs"]
mod render_users;
#[path = "table_pagination.rs"]
mod table_pagination;
#[path = "text_page.rs"]
mod text_page;
pub use admin_ssr_html_try_from_string_error::*;
#[path = "ssr/admin_ssr_text_try_from_string_error.rs"]
mod admin_ssr_text_try_from_string_error;
pub use admin_ssr_text_try_from_string_error::*;
#[path = "ssr/admin_ssr_error_message.rs"]
mod admin_ssr_error_message;
pub use admin_ssr_error_message::*;
#[path = "ssr/admin_ssr_text.rs"]
mod admin_ssr_text;
pub use admin_ssr_text::*;
#[path = "ssr/admin_ssr_html.rs"]
mod admin_ssr_html;
pub use admin_ssr_html::*;
#[path = "ssr/render_user_create.rs"]
mod render_user_create;
pub use render_user_create::*;
#[path = "ssr/render_user_manage.rs"]
mod render_user_manage;
pub use render_user_manage::*;
#[path = "ssr/render_role_create.rs"]
mod render_role_create;
pub use render_role_create::*;
#[path = "ssr/render_role_manage.rs"]
mod render_role_manage;
pub use render_role_manage::*;
#[path = "ssr/render_admin_permissions_page.rs"]
mod render_admin_permissions_page;
pub use render_admin_permissions_page::*;
#[path = "ssr/render_admin_sessions_page.rs"]
mod render_admin_sessions_page;
pub use render_admin_sessions_page::*;
#[path = "ssr/render_admin_profile_page.rs"]
mod render_admin_profile_page;
pub use render_admin_profile_page::*;
#[path = "ssr/render_admin_settings_page.rs"]
mod render_admin_settings_page;
pub use render_admin_settings_page::*;
#[path = "ssr/render_view.rs"]
mod render_view;
use render_view::*;
#[cfg(test)]
#[path = "ssr/admin_ssr_view_ext.rs"]
mod admin_ssr_view_ext;
#[cfg(test)]
use admin_ssr_view_ext::*;

pub use data_tables::render_admin_csr::render_admin_csr;
pub use data_tables::render_data_tables::render_data_tables;
pub use data_tables::render_data_tables_csr::render_data_tables_csr;
pub use document::render_sign_in::render_sign_in;
pub use render_roles::render_roles;
pub use render_users::render_users;
pub use text_page::render_text_page::render_text_page;
pub use text_page::render_text_page_with_access::render_text_page_with_access;

use data_table_grid::data_table_grid;
use document::render_admin_page::render_admin_page;
use document::render_admin_page_with_access::render_admin_page_with_access;
use document::render_admin_page_with_table_access::render_admin_page_with_table_access;
use document::render_document::render_document;
use table_pagination::table_pagination;

#[cfg(test)]
#[path = "domain_types_ssr_tests.rs"]
mod tests;
