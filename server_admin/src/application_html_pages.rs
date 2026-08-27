#[path = "application_html_pages/sign_in_page.rs"]
mod sign_in_page;
pub(super) use sign_in_page::*;
#[path = "application_html_pages/data_tables.rs"]
mod data_tables;
pub(super) use data_tables::*;
#[path = "application_html_pages/users.rs"]
mod users;
pub(super) use users::*;
#[path = "application_html_pages/users_create_page.rs"]
mod users_create_page;
pub(super) use users_create_page::*;
#[path = "application_html_pages/users_manage_page.rs"]
mod users_manage_page;
pub(super) use users_manage_page::*;
#[path = "application_html_pages/roles.rs"]
mod roles;
pub(super) use roles::*;
#[path = "application_html_pages/roles_create_page.rs"]
mod roles_create_page;
pub(super) use roles_create_page::*;
#[path = "application_html_pages/roles_manage_page.rs"]
mod roles_manage_page;
pub(super) use roles_manage_page::*;
#[path = "application_html_pages/permissions.rs"]
mod permissions;
pub(super) use permissions::*;
#[path = "application_html_pages/sessions.rs"]
mod sessions;
pub(super) use sessions::*;
#[path = "application_html_pages/profile.rs"]
mod profile;
pub(super) use profile::*;
#[path = "application_html_pages/settings.rs"]
mod settings;
pub(super) use settings::*;
#[path = "application_html_pages/version.rs"]
mod version;
pub(super) use version::*;
#[path = "application_html_pages/open_api.rs"]
mod open_api;
pub(super) use open_api::*;
#[path = "application_html_pages/router.rs"]
mod router;
pub(super) use router::*;
#[path = "application_html_pages/swagger_router.rs"]
mod swagger_router;
pub(super) use swagger_router::*;
#[path = "application_html_pages/csr_page.rs"]
mod csr_page;
use csr_page::*;
#[path = "application_html_pages/crud_page.rs"]
mod crud_page;
use crud_page::*;
#[path = "application_html_pages/admin_crud_page.rs"]
mod admin_crud_page;
use admin_crud_page::*;
#[path = "application_html_pages/crud_resource_page.rs"]
mod crud_resource_page;
use crud_resource_page::*;
#[path = "application_html_pages/admin_html_page_route_registry.rs"]
mod admin_html_page_route_registry;
use admin_html_page_route_registry::*;
#[path = "application_html_pages/admin_html_swagger_route_registry.rs"]
mod admin_html_swagger_route_registry;
use admin_html_swagger_route_registry::*;
