#![allow(
    unused_imports,
    clippy::unused_trait_names,
    reason = "the CSR loading-shell Leptos view requires attribute traits after macro expansion"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes, StyleAttribute,
};

#[must_use]
#[allow(
    unused_variables,
    reason = "the CSR render contract preserves repository type-based parameter names"
)]
pub fn render_admin_csr(
    admin_bool: server_admin_contract::admin_bool::AdminBool,
    admin_page: server_admin_contract::admin_page::AdminPage,
    option: Option<server_admin_contract::admin_data_table::AdminDataTable>,
    authenticated_admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_branding_view: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let title = admin_branding_view.tab_title().map_or_else(
        || admin_page.spec().title().as_ref().to_owned(),
        |value| value.as_ref().to_owned(),
    );
    let primary_color = admin_branding_view
        .primary_color()
        .map(|value| format!("--accent:{}", value.as_ref()));
    crate::render_document::render_document(
        &crate::admin_ssr_text::AdminSsrText::try_from(title)
            .unwrap_or_else(crate::admin_ssr_text::AdminSsrText::from),
        leptos::view! {
            <div id=constants_str::ADMIN_CSR_ROOT_ID style=primary_color data-password-change-required=bool::from(admin_bool).then_some(constants_str::EMPTY)>
                <crate::admin_spinner::AdminSpinner />
            </div>
            <script type="module" src="/admin/assets/admin_csr_application.js?v=20260906-51"></script>
        },
    )
}
