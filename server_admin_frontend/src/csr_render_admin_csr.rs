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
pub(in crate::domain_types::ssr) fn render_admin_csr(
    page: server_admin_contract::domain_types::AdminPage,
    _active_table: Option<server_admin_contract::domain_types::AdminDataTable>,
    _admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> crate::domain_types::ssr::AdminSsrHtml {
    let title = branding.tab_title().map_or_else(
        || page.spec().title().as_ref().to_owned(),
        |value| value.as_ref().to_owned(),
    );
    let primary_color = branding
        .primary_color()
        .map(|value| format!("--accent:{}", value.as_ref()));
    crate::domain_types::ssr::render_document(
        &crate::domain_types::ssr::AdminSsrText::try_from(title)
            .unwrap_or_else(crate::domain_types::ssr::AdminSsrText::from),
        leptos::view! {
            <div id=constants_str::ADMIN_CSR_ROOT_ID style=primary_color>
                <crate::domain_types::with_owner::admin_spinner::AdminSpinner />
            </div>
            <script type="module" src="/admin/assets/admin_csr_application.js?v=20260801-37"></script>
        },
    )
}
