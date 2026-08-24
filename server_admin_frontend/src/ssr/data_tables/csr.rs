#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the CSR loading-shell Leptos view requires attribute traits after macro expansion"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes, StyleAttribute,
};

#[must_use]
pub(in crate::ssr) fn render_data_tables_csr(
    active_table: Option<server_admin_contract::AdminDataTable>,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> crate::ssr::AdminSsrHtml {
    render_admin_csr(
        server_admin_contract::AdminPage::Tables,
        active_table,
        admin,
        branding,
    )
}

#[must_use]
pub(in crate::ssr) fn render_admin_csr(
    page: server_admin_contract::AdminPage,
    _active_table: Option<server_admin_contract::AdminDataTable>,
    _admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> crate::ssr::AdminSsrHtml {
    let title = branding.tab_title().map_or_else(
        || page.spec().title().as_ref().to_owned(),
        |value| value.as_ref().to_owned(),
    );
    let primary_color = branding
        .primary_color()
        .map(|value| format!("--accent:{}", value.as_ref()));
    crate::ssr::render_document(
        &crate::ssr::AdminSsrText::try_from(title).unwrap_or_else(crate::ssr::AdminSsrText::from),
        leptos::view! {
            <div id=constants_str::ADMIN_CSR_ROOT_ID style=primary_color>
                <crate::ui::spinner::AdminSpinner />
            </div>
            <script type="module" src="/admin/assets/csr_bootstrap.js?v=20260801-37"></script>
        },
    )
}
