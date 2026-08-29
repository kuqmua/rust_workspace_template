#![allow(
    unused_imports,
    reason = "the server-rendered CRUD forms require Leptos attribute traits after macro expansion"
)]

#[allow(
    unused_import_braces,
    reason = "grouped Leptos prelude imports are required by workspace source policy"
)]
#[rustfmt::skip]
use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes};

pub(super) fn crud_render_shell(
    page: server_admin_contract::admin_page::AdminPage,
    content: impl leptos::prelude::IntoAny,
    admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    branding: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let active_table = match page {
        server_admin_contract::admin_page::AdminPage::Users => {
            Some(server_admin_contract::admin_data_table::AdminDataTable::Users)
        }
        server_admin_contract::admin_page::AdminPage::Roles => {
            Some(server_admin_contract::admin_data_table::AdminDataTable::Roles)
        }
        server_admin_contract::admin_page::AdminPage::Metrics
        | server_admin_contract::admin_page::AdminPage::OpenApi
        | server_admin_contract::admin_page::AdminPage::Permissions
        | server_admin_contract::admin_page::AdminPage::Profile
        | server_admin_contract::admin_page::AdminPage::Sessions
        | server_admin_contract::admin_page::AdminPage::Settings
        | server_admin_contract::admin_page::AdminPage::Tables
        | server_admin_contract::admin_page::AdminPage::Version => None,
    };
    crate::render_admin_page_with_table_access::render_admin_page_with_table_access(
        page,
        crate::render_view::render_view(content),
        Some(admin),
        Some(branding),
        active_table,
    )
}
