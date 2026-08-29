pub(crate) use super::admin_html_open_api::*;
pub(crate) use super::admin_html_sessions_page::*;
pub(crate) use super::data_tables::*;
pub(crate) use super::permissions::*;
pub(crate) use super::profile::*;
pub(crate) use super::roles::*;
pub(crate) use super::roles_create_page::*;
pub(crate) use super::roles_manage_page::*;
pub(crate) use super::settings::*;
pub(crate) use super::sign_in_page::*;
pub(crate) use super::users::*;
pub(crate) use super::users_create_page::*;
pub(crate) use super::users_manage_page::*;
pub(crate) use super::version::*;
use admin_crud_page::*;
use admin_html_page_route_registry::*;
use admin_html_swagger_route_registry::*;
use crud_page::*;
use crud_resource_page::*;
use csr_page::*;
// Root-owned module compatibility wrappers.
mod sign_in_page {
    pub use super::super::sign_in_page::*;
}
mod data_tables {
    pub use super::super::data_tables::*;
}
mod users {
    pub use super::super::users::*;
}
mod users_create_page {
    pub use super::super::users_create_page::*;
}
mod users_manage_page {
    pub use super::super::users_manage_page::*;
}
mod roles {
    pub use super::super::roles::*;
}
mod roles_create_page {
    pub use super::super::roles_create_page::*;
}
mod roles_manage_page {
    pub use super::super::roles_manage_page::*;
}
mod permissions {
    pub use super::super::permissions::*;
}
mod admin_html_sessions_page {
    pub use super::super::admin_html_sessions_page::*;
}
mod profile {
    pub use super::super::profile::*;
}
mod settings {
    pub use super::super::settings::*;
}
mod version {
    pub use super::super::version::*;
}
mod admin_html_open_api {
    pub use super::super::admin_html_open_api::*;
}
mod csr_page {
    pub use super::super::csr_page::*;
}
mod crud_page {
    pub use super::super::crud_page::*;
}
mod admin_crud_page {
    pub use super::super::admin_crud_page::*;
}
mod crud_resource_page {
    pub use super::super::crud_resource_page::*;
}
mod admin_html_page_route_registry {
    pub use super::super::admin_html_page_route_registry::*;
}
mod admin_html_swagger_route_registry {
    pub use super::super::admin_html_swagger_route_registry::*;
}
