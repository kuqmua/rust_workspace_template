use admin_crud_page::*;
pub(crate) use admin_html_open_api::*;
use admin_html_page_route_registry::*;
pub(crate) use admin_html_sessions_page::*;
use admin_html_swagger_route_registry::*;
use crud_page::*;
use crud_resource_page::*;
use csr_page::*;
pub(crate) use data_tables::*;
pub(crate) use permissions::*;
pub(crate) use profile::*;
pub(crate) use roles::*;
pub(crate) use roles_create_page::*;
pub(crate) use roles_manage_page::*;
pub(crate) use settings::*;
pub(crate) use sign_in_page::*;
pub(crate) use users::*;
pub(crate) use users_create_page::*;
pub(crate) use users_manage_page::*;
pub(crate) use version::*;

// Root-owned module compatibility wrappers.
mod sign_in_page {
    pub use crate::sign_in_page::*;
}
mod data_tables {
    pub use crate::data_tables::*;
}
mod users {
    pub use crate::users::*;
}
mod users_create_page {
    pub use crate::users_create_page::*;
}
mod users_manage_page {
    pub use crate::users_manage_page::*;
}
mod roles {
    pub use crate::roles::*;
}
mod roles_create_page {
    pub use crate::roles_create_page::*;
}
mod roles_manage_page {
    pub use crate::roles_manage_page::*;
}
mod permissions {
    pub use crate::permissions::*;
}
mod admin_html_sessions_page {
    pub use crate::admin_html_sessions_page::*;
}
mod profile {
    pub use crate::profile::*;
}
mod settings {
    pub use crate::settings::*;
}
mod version {
    pub use crate::version::*;
}
mod admin_html_open_api {
    pub use crate::admin_html_open_api::*;
}
mod csr_page {
    pub use crate::csr_page::*;
}
mod crud_page {
    pub use crate::crud_page::*;
}
mod admin_crud_page {
    pub use crate::admin_crud_page::*;
}
mod crud_resource_page {
    pub use crate::crud_resource_page::*;
}
mod admin_html_page_route_registry {
    pub use crate::admin_html_page_route_registry::*;
}
mod admin_html_swagger_route_registry {
    pub use crate::admin_html_swagger_route_registry::*;
}
