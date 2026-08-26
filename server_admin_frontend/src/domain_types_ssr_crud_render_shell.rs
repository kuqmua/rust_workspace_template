#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the server-rendered CRUD forms require Leptos attribute traits after macro expansion"
)]

#[allow(
    unused_import_braces,
    reason = "grouped Leptos prelude imports are required by workspace source policy"
)]
#[rustfmt::skip]
use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes};

pub(super) fn crud_render_shell(
    page: server_admin_contract::domain_types::AdminPage,
    content: impl leptos::prelude::IntoAny,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> super::AdminSsrHtml {
    let active_table = match page {
        server_admin_contract::domain_types::AdminPage::Users => {
            Some(server_admin_contract::domain_types::AdminDataTable::Users)
        }
        server_admin_contract::domain_types::AdminPage::Roles => {
            Some(server_admin_contract::domain_types::AdminDataTable::Roles)
        }
        server_admin_contract::domain_types::AdminPage::Metrics
        | server_admin_contract::domain_types::AdminPage::OpenApi
        | server_admin_contract::domain_types::AdminPage::Permissions
        | server_admin_contract::domain_types::AdminPage::Profile
        | server_admin_contract::domain_types::AdminPage::Sessions
        | server_admin_contract::domain_types::AdminPage::Settings
        | server_admin_contract::domain_types::AdminPage::Tables
        | server_admin_contract::domain_types::AdminPage::Version => None,
    };
    super::render_admin_page_with_table_access(
        page,
        super::render_view(content),
        Some(admin),
        Some(branding),
        active_table,
    )
}
