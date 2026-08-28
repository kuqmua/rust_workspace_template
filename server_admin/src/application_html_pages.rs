#![allow(clippy::wildcard_imports)] // split page modules share a private facade vocabulary
#[path = "sign_in_page.rs"]
mod sign_in_page;
pub(super) use sign_in_page::*;
#[path = "data_tables.rs"]
mod data_tables;
pub(super) use data_tables::*;
#[path = "users.rs"]
mod users;
pub(super) use users::*;
#[path = "users_create_page.rs"]
mod users_create_page;
pub(super) use users_create_page::*;
#[path = "users_manage_page.rs"]
mod users_manage_page;
pub(super) use users_manage_page::*;
#[path = "roles.rs"]
mod roles;
pub(super) use roles::*;
#[path = "roles_create_page.rs"]
mod roles_create_page;
pub(super) use roles_create_page::*;
#[path = "roles_manage_page.rs"]
mod roles_manage_page;
pub(super) use roles_manage_page::*;
#[path = "permissions.rs"]
mod permissions;
pub(super) use permissions::*;
#[path = "admin_html_sessions_page.rs"]
mod admin_html_sessions_page;
pub(super) use admin_html_sessions_page::*;
#[path = "profile.rs"]
mod profile;
pub(super) use profile::*;
#[path = "settings.rs"]
mod settings;
pub(super) use settings::*;
#[path = "version.rs"]
mod version;
pub(super) use version::*;
#[path = "admin_html_open_api.rs"]
mod admin_html_open_api;
pub(super) use admin_html_open_api::*;
#[path = "admin_html_page_router.rs"]
mod admin_html_page_router;
pub(super) use admin_html_page_router::*;
#[path = "swagger_router.rs"]
mod swagger_router;
pub(super) use swagger_router::*;
#[path = "csr_page.rs"]
mod csr_page;
use csr_page::*;
#[path = "crud_page.rs"]
mod crud_page;
use crud_page::*;
#[path = "admin_crud_page.rs"]
mod admin_crud_page;
use admin_crud_page::*;
#[path = "crud_resource_page.rs"]
mod crud_resource_page;
use crud_resource_page::*;
#[path = "admin_html_page_route_registry.rs"]
mod admin_html_page_route_registry;
use admin_html_page_route_registry::*;
#[path = "admin_html_swagger_route_registry.rs"]
mod admin_html_swagger_route_registry;
use admin_html_swagger_route_registry::*;
